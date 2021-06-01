use super::entities::route;
use super::{
    access, commands, connectors, entities, errors, providers, replies, schema, traits::ObjectType,
};
use hyper::Body;
use bytes::Buf;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

pub const ROUTER_SERVICE_NAME: &str = "router";

pub struct Router {
    data_connector: Option<Arc<connectors::DataConnector>>,
    access_checker: Option<Arc<access::AccessChecker>>,
    http_provider: providers::HttpProvider,
    remote_router: Option<HashMap<String, String>>,
    service_path: RwLock<HashMap<String, HashMap<String, route::ServicePath>>>,
    command: RwLock<HashMap<String, entities::route::CommandRoute>>,
    subscription: RwLock<HashMap<String, Vec<entities::route::SubscriptionRoute>>>,
    pub schema: HashMap<&'static str, schemars::schema::RootSchema>,
    pub is_local: bool,
}

impl Router {
    fn make_service_path_hash_map(
        paths: Vec<route::ServicePath>,
    ) -> HashMap<String, HashMap<String, route::ServicePath>> {
        let mut rhm = HashMap::<String, HashMap<String, route::ServicePath>>::new();
        let mut shm = HashMap::<String, route::ServicePath>::new();
        let mut lsn = None;
        for item in paths {
            if lsn.is_none() {
                lsn = item.service_name.clone();
            }
            if lsn.as_ref().unwrap() != item.service_name.as_ref().unwrap() {
                rhm.insert(lsn.unwrap().clone(), shm.clone());
                shm.clear();
            }
            shm.insert(
                item.proto.as_ref().unwrap().to_string(),
                entities::route::ServicePath {
                    service_name: item.service_name.clone(),
                    proto: item.proto,
                    helth: item.helth,
                    schema: item.schema,
                    reply_to: item.reply_to,
                    state: item.state,
                    error: item.error,
                    request: item.request,
                    event: item.event,
                },
            );
            lsn = Some(item.service_name.unwrap());
        }
        if shm.len() > 0 {
            rhm.insert(lsn.unwrap(), shm);
        }
        if rhm.len() > 0 {
            debug!("{} service paths", rhm.len());
        } else {
            warn!("{} service paths", rhm.len());
        }
        rhm
    }

    fn make_command_hash_map(
        commands: Vec<route::ServiceCommand>,
    ) -> HashMap<String, entities::route::CommandRoute> {
        let mut hm = HashMap::<String, entities::route::CommandRoute>::new();
        let mut lot = None;
        for item in commands {
            if lot.is_none() || lot.unwrap() != item.object_type {
                let mut st = HashMap::<String, String>::new();
                if item.state.is_some() {
                    st = item.state.unwrap();
                }
                hm.insert(
                    item.object_type.clone(),
                    entities::route::CommandRoute {
                        object_type: item.object_type.clone(),
                        reply_type: item.reply_type.clone(),
                        exec_mode: item.exec_mode,
                        state: st,
                        service_name: item.service_name,
                        path: item.path.unwrap(),
                    },
                );
            }
            lot = Some(item.object_type);
        }
        if hm.len() > 0 {
            debug!("{} commands", hm.len());
        } else {
            warn!("{} commands", hm.len());
        }
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
        if hm.len() > 0 {
            debug!("{} subscriptions", hm.len());
        } else {
            warn!("{} subscriptions", hm.len());
        }
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
                    exec_mode: c.exec_mode.clone(),
                    reply_type: c.reply_type.clone(),
                    path: np,
                    state: c.state.clone()
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
        let is_local = remote_router.is_none();
        let hp = providers::HttpProvider::new().await?;
        let mut _service_paths = Vec::<route::ServicePath>::new();
        let mut _commands = Vec::<route::ServiceCommand>::new();
        let mut _subscriptions = Vec::<route::ServiceSubscription>::new();
        if is_local {
            &dc.route.add(service.values().cloned().collect()).await?;
            let p = &dc.route.get_service_path(None).await?;
            let c = &dc.route.get_command(None).await?;
            let s = &dc.route.get_subscription(None).await?;
            _service_paths = p.to_vec();
            _commands = c.to_vec();
            _subscriptions = s.to_vec();
        } else {
            if !remote_router
                .as_ref()
                .unwrap()
                .contains_key(&providers::Proto::http.to_string())
            {
                return Err(errors::UnsupportedProtoError.into());
            }
            let r = remote_router
                .as_ref()
                .unwrap()
                .get(&providers::Proto::http.to_string())
                .unwrap();
            let cid = Uuid::new_v4().to_hyphenated().to_string();
            let mut prop = HashMap::<&str, &str>::new();
            prop.insert("correlation_id", &cid);
            prop.insert(
                "object_type",
                commands::route::GetServiceCommand::get_type_name(),
            );
            let token =
                &ac.get_client_basic_authorization_token(&ROUTER_SERVICE_NAME.to_string())?;
            //todo: add routes
            //todo: get service path
            //get commands
            let resp_command = hp
                .execute(
                    r,
                    prop,
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
            let cid = Uuid::new_v4().to_hyphenated().to_string();
            let mut prop = HashMap::<&str, &str>::new();
            prop.insert("correlation_id", &cid);
            prop.insert(
                "object_type",
                commands::route::GetServiceSubscription::get_type_name(),
            );
            //get subscriptions
            let resp_subscription = hp
                .execute(
                    r,
                    prop,
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
                _commands = reply_command
                    .as_ref()
                    .unwrap()
                    .items
                    .as_ref()
                    .unwrap()
                    .to_vec();
                _subscriptions = reply_subscription
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
            data_connector: if is_local { Some(dc) } else { None },
            access_checker: if is_local { None } else { Some(ac) },
            http_provider: hp,
            schema: schema::make_schema(),
            remote_router: remote_router,
            service_path: RwLock::new(Router::make_service_path_hash_map(_service_paths)),
            command: RwLock::new(Router::make_command_hash_map(_commands)),
            subscription: RwLock::new(Router::make_subscription_hash_map(_subscriptions)),
            is_local: is_local,
        })
    }

    fn validate(
        service_paths: &Vec<route::ServicePath>,
        commands: &Vec<route::ServiceCommand>,
        subscriptions: &Vec<route::ServiceSubscription>,
    ) -> connectors::Result<bool> {
        //todo: check data integrity
        Ok(true)
    }

    pub async fn update(
        &self,
        service_paths: Vec<route::ServicePath>,
        commands: Vec<route::ServiceCommand>,
        subscriptions: Vec<route::ServiceSubscription>,
    ) -> connectors::Result<bool> {
        if Router::validate(&service_paths, &commands, &subscriptions)? {
            let mut sp = self.service_path.write().unwrap();
            let mut cm = self.command.write().unwrap();
            let mut ss = self.subscription.write().unwrap();
            *sp = Router::make_service_path_hash_map(service_paths);
            *cm = Router::make_command_hash_map(commands);
            *ss = Router::make_subscription_hash_map(subscriptions);
            Ok(true)
        } else {
            Err(errors::DataIntegrityError.into())
        }
    }

    pub fn get_service_path(
        &self,
        service_name: &str,
        proto: providers::Proto,
    ) -> connectors::Result<entities::route::ServicePath> {
        let sp = self.service_path.read().unwrap();
        if sp.contains_key(service_name) {
            let sn = sp.get(service_name).unwrap();
            if sn.contains_key(&proto.to_string()) {
                Ok(sn.get(&proto.to_string()).unwrap().clone())
            } else {
                Err(errors::UnsupportedProtoError.into())
            }
        } else {
            Err(errors::UnknownServiceNameError.into())
        }
    }

    pub fn get_command(
        &self,
        object_type: &str,
    ) -> connectors::Result<entities::route::CommandRoute> {
        if self.command.read().unwrap().contains_key(object_type) {
            Ok(self
                .command
                .read()
                .unwrap()
                .get(object_type)
                .unwrap()
                .clone())
        } else {
            Err(errors::UnknownCommandError.into())
        }
    }

    pub fn get_subscriptions(
        &self,
        object_type: &str,
    ) -> Option<Vec<entities::route::SubscriptionRoute>> {
        if self.subscription.read().unwrap().contains_key(object_type) {
            Some(
                self.subscription
                    .read()
                    .unwrap()
                    .get(object_type)
                    .unwrap()
                    .clone(),
            )
        } else {
            None
        }
    }

    pub async fn shutdown(&self) -> connectors::Result<()> {
        let mut s = Vec::<String>::new();
        for item in self.service_path.read().unwrap().keys() {
            s.push(item.to_string());
        }
        if self.is_local {
            self.data_connector
                .as_ref()
                .unwrap()
                .route
                .remove(s)
                .await?;
            debug!("remove service route");
        } else {
            let r = self
                .remote_router
                .as_ref()
                .unwrap()
                .get(&providers::Proto::http.to_string())
                .unwrap();
            let cid = Uuid::new_v4().to_hyphenated().to_string();
            let mut prop = HashMap::<&str, &str>::new();
            prop.insert("correlation_id", &cid);
            prop.insert("object_type", commands::route::RemoveRoute::get_type_name());
            let token = self
                .access_checker
                .as_ref()
                .unwrap()
                .get_client_basic_authorization_token(&ROUTER_SERVICE_NAME.to_string())?;
            let resp = self
                .http_provider
                .execute(
                    r,
                    prop,
                    token.to_string(),
                    Body::from(
                        serde_json::to_string(&commands::route::RemoveRoute { services: s })
                            .unwrap(),
                    ),
                )
                .await?;
            let reader = hyper::body::aggregate(resp).await?.reader();
            let reply: Option<replies::common::StandardReply> =
                serde_json::from_reader(reader).unwrap_or(None);
            if reply.is_some() && reply.as_ref().unwrap().error_code == errors::ErrorCode::ReplyOk {
                debug!("remove service route");
            } else {
                warn!("some errors while remove service route");
            }
        }
        Ok({})
    }
}
