use super::entities::route;
use super::{
    access, commands, connectors, entities, errors, providers, replies, schema, traits::ObjectType,
};
use bytes::buf::BufExt;
use hyper::Body;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

pub const ROUTER_SERVICE_NAME: &str = "router";

pub struct Router {
    dc: Option<Arc<connectors::DataConnector>>,
    ac: Option<Arc<access::AccessChecker>>,
    hp: providers::HttpProvider,
    sv: HashMap<String, HashMap<String, route::ServicePath>>,
    rr: Option<HashMap<String, String>>,
    chm: RwLock<HashMap<String, entities::route::CommandRoute>>,
    shm: RwLock<HashMap<String, Vec<entities::route::SubscriptionRoute>>>,
    pub schema: HashMap<&'static str, schemars::schema::RootSchema>,
    pub is_local: bool,
}

impl Router {
    fn make_command_hash_map(
        commands: Vec<route::ServiceCommand>,
    ) -> HashMap<String, entities::route::CommandRoute> {
        let mut hm = HashMap::<String, entities::route::CommandRoute>::new();
        let mut lot = None;
        for item in commands {
            if lot.is_none() || lot.unwrap() != item.object_type {
                hm.insert(
                    item.object_type.clone(),
                    entities::route::CommandRoute {
                        object_type: item.object_type.clone(),
                        reply_type: item.reply_type.clone(),
                        service_name: item.service_name,
                        path: item.path.unwrap(),
                    },
                );
            }
            lot = Some(item.object_type);
        }
        debug!("{} commands", hm.len());
        hm
    }

    fn make_subscription_hash_map(
        subscriptions: Vec<route::ServiceSubscription>,
    ) -> HashMap<String, Vec<entities::route::SubscriptionRoute>> {
        let mut hm = HashMap::<String, Vec<entities::route::SubscriptionRoute>>::new();
        let mut sr = Vec::<entities::route::SubscriptionRoute>::new();
        let mut lot = None;
        for item in subscriptions {
            if lot.is_none() {
                lot = Some(item.object_type.clone());
            }
            if lot.as_ref().unwrap() != &item.object_type {
                hm.insert(lot.unwrap().clone(), sr.clone());
                sr.clear();
            }
            sr.push(entities::route::SubscriptionRoute {
                object_type: item.object_type.clone(),
                service_name: item.service_name,
                path: item.path.unwrap(),
            });
            lot = Some(item.object_type);
        }
        if sr.len() > 0 {
            hm.insert(lot.unwrap(), sr);
        }
        debug!("{} subscriptions", hm.len());
        hm
    }

    pub fn update_host_mask(host: &str, path: &mut route::ServicePath) {
        path.helth = path.helth.replace("{host}", host);
        path.schema = path.schema.replace("{host}", host);
        path.reply_to = path.reply_to.replace("{host}", host);
        path.error = path.error.replace("{host}", host);
        path.request = Some(path.request.as_ref().unwrap().replace("{host}", host));
        path.event = Some(path.event.as_ref().unwrap().replace("{host}", host));
    }

    pub async fn new(
        dc: Arc<connectors::DataConnector>,
        ac: Arc<access::AccessChecker>,
        remote_router: Option<HashMap<String, String>>,
        mut path: HashMap<String, route::ServicePath>,
        mut service: HashMap<String, route::Route>,
        host: &str,
    ) -> connectors::Result<Router> {
        for item in service.iter_mut() {
            item.1.service_name = Some(item.0.to_string());
        }
        let mut root_command_path = HashMap::<String, String>::new();
        let mut root_subscription_path = HashMap::<String, String>::new();
        for p in path.iter_mut() {
            Router::update_host_mask(host, p.1);
            root_command_path.insert(p.0.to_string(), p.1.request.as_ref().unwrap().to_string());
            root_subscription_path.insert(p.0.to_string(), p.1.event.as_ref().unwrap().to_string());
        }
        for item in service.values_mut() {
            let (service_command_path, service_subscription_path) = if item.path.is_none() {
                item.path = Some(path.clone());
                (root_command_path.clone(), root_subscription_path.clone())
            } else {
                let mut chm = HashMap::<String, String>::new();
                let mut shm = HashMap::<String, String>::new();
                for p in item.path.as_mut().unwrap().iter_mut() {
                    Router::update_host_mask(host, p.1);
                    chm.insert(p.0.to_string(), p.1.request.as_ref().unwrap().to_string());
                    shm.insert(p.0.to_string(), p.1.event.as_ref().unwrap().to_string());
                }
                (chm, shm)
            };
            let mut nc = Vec::<route::ServiceCommand>::new();
            for c in &item.command {
                let np = Some(if c.path.is_some() {
                    let mut cp = c.path.as_ref().unwrap().clone();
                    for p in cp.values_mut() {
                        *p = p.replace("{host}", host);
                    }
                    cp
                } else {
                    service_command_path.clone()
                });
                nc.push(route::ServiceCommand {
                    service_name: None,
                    priority: None,
                    object_type: c.object_type.clone(),
                    description: c.description.clone(),
                    reply_type: c.reply_type.clone(),
                    path: np,
                });
            }
            item.command = nc;
            let mut ns = Vec::<route::ServiceSubscription>::new();
            for s in &item.subscription {
                let np = Some(if s.path.is_some() {
                    let mut sp = s.path.as_ref().unwrap().clone();
                    for p in sp.values_mut() {
                        *p = p.replace("{host}", host);
                    }
                    sp
                } else {
                    service_subscription_path.clone()
                });
                ns.push(route::ServiceSubscription {
                    service_name: None,
                    object_type: s.object_type.clone(),
                    path: np,
                });
            }
            item.subscription = ns;
        }
        let mut sv = HashMap::<String, HashMap<String, route::ServicePath>>::new();
        for item in service.iter() {
            sv.insert(item.0.to_string(), item.1.path.as_ref().unwrap().clone());
        }
        let is_local = remote_router.is_none();
        let hp = providers::HttpProvider::new().await?;
        let mut commands = Vec::<route::ServiceCommand>::new();
        let mut subscriptions = Vec::<route::ServiceSubscription>::new();
        if is_local {
            &dc.route.add(service.values().cloned().collect()).await?;
            let gc = &dc.route.get_command(None).await?;
            let gs = &dc.route.get_subscription(None).await?;
            commands = gc.to_vec();
            subscriptions = gs.to_vec();
        } else {
            if !remote_router
                .as_ref()
                .unwrap()
                .contains_key(connectors::PROTO_HTTP)
            {
                return Err(errors::SupportedtProtoNotFoundError.into());
            }
            let r = remote_router
                .as_ref()
                .unwrap()
                .get(connectors::PROTO_HTTP)
                .unwrap();
            let token =
                &ac.get_client_basic_authorization_token(ROUTER_SERVICE_NAME.to_string())?;
            let correlation_id_command = Uuid::new_v4().to_hyphenated().to_string();
            let resp_command = hp
                .execute(
                    r,
                    commands::route::GetServiceCommand::get_type_name(),
                    &correlation_id_command,
                    token.to_string(),
                    Body::from(
                        serde_json::to_string(&commands::route::GetServiceCommand {
                            filter: None,
                            services: None,
                        })
                        .unwrap(),
                    ),
                )
                .await?;
            let reader_command = hyper::body::aggregate(resp_command).await?.reader();
            let reply_command: Option<replies::route::GetServiceCommandReply> =
                serde_json::from_reader(reader_command).unwrap_or(None);
            let correlation_id_subscription = Uuid::new_v4().to_hyphenated().to_string();
            let resp_subscription = hp
                .execute(
                    r,
                    commands::route::GetServiceSubscription::get_type_name(),
                    &correlation_id_subscription,
                    token.to_string(),
                    Body::from(
                        serde_json::to_string(&commands::route::GetServiceSubscription {
                            filter: None,
                            services: None,
                        })
                        .unwrap(),
                    ),
                )
                .await?;
            let reader_subscription = hyper::body::aggregate(resp_subscription).await?.reader();
            let reply_subscription: Option<replies::route::GetServiceSubscriptionReply> =
                serde_json::from_reader(reader_subscription).unwrap_or(None);
            if reply_command.is_some()
                && reply_subscription.is_some()
                && reply_command.as_ref().unwrap().error_code == errors::ErrorCode::ReplyOk
                && reply_subscription.as_ref().unwrap().error_code == errors::ErrorCode::ReplyOk
            {
                commands = reply_command
                    .as_ref()
                    .unwrap()
                    .items
                    .as_ref()
                    .unwrap()
                    .to_vec();
                subscriptions = reply_subscription
                    .as_ref()
                    .unwrap()
                    .items
                    .as_ref()
                    .unwrap()
                    .to_vec();
            } else {
                return Err(errors::GeRemoteRouterError.into());
            }
        }
        Ok(Router {
            dc: if is_local { Some(dc) } else { None },
            ac: if is_local { None } else { Some(ac) },
            hp: hp,
            schema: schema::make_schema(),
            rr: remote_router,
            sv: sv,
            chm: RwLock::new(Router::make_command_hash_map(commands.to_vec())),
            shm: RwLock::new(Router::make_subscription_hash_map(subscriptions.to_vec())),
            is_local: is_local,
        })
    }

    pub async fn update(
        &self,
        commands: Vec<route::ServiceCommand>,
        subscriptions: Vec<route::ServiceSubscription>,
    ) -> connectors::Result<bool> {
        let mut chm = self.chm.write().unwrap();
        let mut shm = self.shm.write().unwrap();
        *chm = Router::make_command_hash_map(commands);
        *shm = Router::make_subscription_hash_map(subscriptions);
        Ok(true)
    }

    pub fn get_command(
        &self,
        object_type: &str,
    ) -> connectors::Result<entities::route::CommandRoute> {
        if self.chm.read().unwrap().contains_key(object_type) {
            Ok(self.chm.read().unwrap().get(object_type).unwrap().clone())
        } else {
            Err(errors::UnknownCommandError.into())
        }
    }

    pub fn get_subscriptions(
        &self,
        object_type: &str,
    ) -> Option<Vec<entities::route::SubscriptionRoute>> {
        if self.shm.read().unwrap().contains_key(object_type) {
            Some(self.shm.read().unwrap().get(object_type).unwrap().clone())
        } else {
            None
        }
    }

    pub async fn shutdown(&self) -> connectors::Result<()> {
        let mut s = Vec::<String>::new();
        for item in self.sv.keys() {
            s.push(item.to_string());
        }
        if self.is_local {
            self.dc.as_ref().unwrap().route.remove(s).await?;
            debug!("remove service route");
        } else {
            let r = self
                .rr
                .as_ref()
                .unwrap()
                .get(connectors::PROTO_HTTP)
                .unwrap();
            let token = self
                .ac
                .as_ref()
                .unwrap()
                .get_client_basic_authorization_token(ROUTER_SERVICE_NAME.to_string())?;
            let correlation_id = Uuid::new_v4().to_hyphenated().to_string();
            let resp = self
                .hp
                .execute(
                    r,
                    commands::route::RemoveRoute::get_type_name(),
                    &correlation_id,
                    token.to_string(),
                    Body::from(
                        serde_json::to_string(&commands::route::RemoveRoute { ids: s }).unwrap(),
                    ),
                )
                .await?;
            let reader = hyper::body::aggregate(resp).await?.reader();
            let reply: Option<replies::common::StandardReply> =
                serde_json::from_reader(reader).unwrap_or(None);
            if reply.is_some() && reply.as_ref().unwrap().error_code == errors::ErrorCode::ReplyOk {
                debug!("remove service route");
            }
            else{
                warn!("some errors while remove service route");
            }
        }
        Ok({})
    }
}
