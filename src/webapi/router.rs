use super::entities::route;
use super::{connectors, entities, errors, schema, settings};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

pub struct Router {
    service: HashMap<String, HashMap<String, route::ServicePath>>,
    chm: RwLock<HashMap<String, entities::route::CommandRoute>>,
    shm: RwLock<HashMap<String, Vec<entities::route::SubscriptionRoute>>>,
    pub schema: HashMap<&'static str, schemars::schema::RootSchema>,
    pub is_local: bool,
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
        dc: &connectors::DataConnector,
        router: Option<HashMap<String, String>>,
        mut path: HashMap<String, route::ServicePath>,
        mut service: HashMap<String, route::Route>,
        host: &str,
    ) -> connectors::Result<Router> {
        for p in path.values_mut() {
            Router::update_host_mask(host, p);
        }
        for item in service.iter_mut() {
            item.1.service_name = Some(item.0.to_string());
        }
        for item in service.values_mut() {
            if item.path.is_none() {
                item.path = Some(path.clone());
            }
        }
        let mut sv = HashMap::<String, HashMap<String, route::ServicePath>>::new();
        for item in service.iter() {
            sv.insert(item.0.to_string(), item.1.path.as_ref().unwrap().clone());
        }
        let is_local = router.is_some();
        if is_local{
            dc.route.add(service.values().cloned().collect()).await?;
        }
        else{
            //call remote route using self.router hashmap with proto/to with command AddRoute
        }
        Ok(Router {
            schema: schema::make_schema(),
            service: sv,
            chm: RwLock::new(Router::make_command_hash_map(
                dc.route.get_command(None).await?,
            )),
            shm: RwLock::new(Router::make_subscription_hash_map(
                dc.route.get_subscription(None).await?,
            )),
            is_local: is_local,
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
        let mut s = Vec::<String>::new();
        for item in self.service.keys() {
            s.push(item.to_string());
        }
        if self.is_local {
            dc.route.remove(s).await?;
        }
        else{
            //call remote router with command RemoveRoute
        }
        debug!("remove service route");
        Ok({})
    }
}
