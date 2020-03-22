use super::entities::route;
use super::{connectors, entities, errors};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

pub struct Router {
    sn: String,
    loc_rt_sn: Option<String>,
    chm: RwLock<HashMap<String, entities::route::CommandRoute>>,
    shm: RwLock<HashMap<String, Vec<entities::route::SubscriptionRoute>>>,
}

impl Router {
    fn make_command_hash_map(
        commands: Vec<route::Command>,
    ) -> HashMap<String, entities::route::CommandRoute> {
        let mut hm = HashMap::<String, entities::route::CommandRoute>::new();
        let mut lot = None;
        for item in commands {
            if lot.is_none() || lot.unwrap() != item.object_type {
                hm.insert(
                    item.object_type.clone(),
                    entities::route::CommandRoute {
                        object_type: item.object_type.clone(),
                        service_name: item.service_name,
                        http_to: item.http_to,
                        mq_to: item.mq_to,
                    },
                );
            }
            lot = Some(item.object_type);
        }
        debug!("{} commands", hm.len());
        hm
    }

    fn make_subscription_hash_map(
        subscriptions: Vec<route::Subscription>,
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
                http_to: item.http_to,
                mq_to: item.mq_to,
            });
            lot = Some(item.object_type);
        }
        if sr.len() > 0 {
            hm.insert(lot.unwrap(), sr);
        }
        debug!("{} subscriptions", hm.len());
        hm
    }

    fn update_host_mask(host: &str, service: &mut route::Route) {
        for c in &mut service.command {
            if c.http_to.is_some() {
                let v = c.http_to.as_ref().unwrap();
                c.http_to = Some(v.replace("{host}", host));
            }
        }
        for s in &mut service.subscription {
            if s.http_to.is_some() {
                let v = s.http_to.as_ref().unwrap();
                s.http_to = Some(v.replace("{host}", host));
            }
        }
        service.http_helth = service.http_helth.replace("{host}", host);
    }

    pub fn is_local(&self) -> bool {
        self.loc_rt_sn.is_some()
    }

    pub async fn new_remote(
        _http_from: Option<String>,
        _mq_from: Option<String>,
        mut service: route::Route,
        host: &str,
    ) -> connectors::Result<Router> {
        Router::update_host_mask(host, &mut service);
        let c = Vec::<entities::route::Command>::new();
        let s = Vec::<entities::route::Subscription>::new();
        Ok(Router {
            sn: service.service_name,
            loc_rt_sn: None,
            chm: RwLock::new(Router::make_command_hash_map(c)),
            shm: RwLock::new(Router::make_subscription_hash_map(s)),
        })
    }

    pub async fn new_local(
        dc: &connectors::DataConnector,
        mut service: route::Route,
        mut local_router: route::Route,
        host: &str,
    ) -> connectors::Result<Router> {
        Router::update_host_mask(host, &mut service);
        Router::update_host_mask(host, &mut local_router);
        let sn = service.service_name.clone();
        let loc_rt_sn = local_router.service_name.clone();
        dc.route.add(vec![local_router]).await?;
        debug!("add local route");
        dc.route.add(vec![service]).await?;
        debug!("add service route");
        Ok(Router {
            sn: sn,
            loc_rt_sn: Some(loc_rt_sn),
            chm: RwLock::new(Router::make_command_hash_map(
                dc.route.get_command(None).await?,
            )),
            shm: RwLock::new(Router::make_subscription_hash_map(
                dc.route.get_subscription(None).await?,
            )),
        })
    }

    pub async fn update(
        &self,
        commands: Vec<route::Command>,
        subscriptions: Vec<route::Subscription>,
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

    pub async fn shutdown(&self, dc: Arc<connectors::DataConnector>) -> connectors::Result<()> {
        if self.loc_rt_sn.is_some() {
            dc.route
                .remove(vec![self.loc_rt_sn.as_ref().unwrap().to_string()])
                .await?;
            debug!("remove local route");
        }
        dc.route.remove(vec![self.sn.clone()]).await?;
        debug!("remove service route");
        Ok({})
    }
}
