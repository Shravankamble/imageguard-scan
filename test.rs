// // // use serde::{Serialize};
// // // use serde_json::{Value};

// // // #[allow(non_camel_case_types)]
// // // #[derive(Debug, Serialize)]
// // // pub struct Status {
// // //     pub code: i16,
// // //     pub message: String,
// // // }

// // // #[allow(non_camel_case_types)]
// // // #[derive(Debug, Serialize)]
// // // pub struct Response {
// // //     pub uid: Value,
// // //     pub allowed: bool,
// // //     pub status: Status,
// // // }

// // // #[allow(non_snake_case)]
// // // #[derive(Debug, Serialize)]
// // // pub struct AdmissionReview {
// // //     pub apiVersion: String,
// // //     pub kind: String,
// // //     pub response: Response,
// // // }

// // // let _ = tokio::spawn(async move {
// // //         loop {
// // //             while let Some(event) = events.try_next().await.unwrap() {
// // //                 if event.name_any() == "my-pod".to_string() {
// // //                     // tags.insert(event.clone().name_any());
// // //                     // forbidden_tags.insert("pod".to_string(), tags);
// // //                     // tags.remove(&event.name_any());
// // //                     log::info!("my pod created")
// // //                 } else {
// // //                     log::info!("some other pod created")
// // //                 }
// // //             }
// // //         }
// // //     });


// // use axum::{
// //     Extension, Json, Router,
// //     body::{Body, Bytes, to_bytes},
// //     extract::{Request, State},
// //     http::{Error, Response, StatusCode},
// //     middleware::{self, Next},
// //     response::IntoResponse,
// //     routing::{get, post},
// // };
// // use crd::crd::{Tag, TagSpec, TagStatus};
// // use futures::{StreamExt, TryStreamExt, lock::Mutex};
// // use k8s_openapi::{ByteString, api::core::v1::Pod};
// // use kube::{
// //     Api, CustomResource, ResourceExt,
// //     api::{ListParams, WatchEvent},
// //     core::{
// //         admission::{self, AdmissionRequest},
// //         response::StatusCause,
// //     },
// //     runtime::{
// //         WatchStreamExt,
// //         watcher::{self, Event, watcher},
// //     },
// // };
// // mod cache;
// // use crate::cache::cache::ImageCache;
// // use log::{info, warn};
// // use moka::future::Cache;
// // use prometheus_client::{
// //     encoding::{DescriptorEncoder, EncodeLabelSet, EncodeLabelValue, text::encode},
// //     metrics::{counter::Counter, family::Family},
// //     registry::{self, Registry},
// // };
// // use schemars::JsonSchema;
// // use serde::{Deserialize, Serialize};
// // use serde_json::{Value, to_string};
// // use std::{
// //     collections::HashSet,
// //     sync::{Arc, RwLock},
// // };
// // use tokio::net::TcpListener;
// // use tower::{ServiceBuilder, buffer};

// // #[allow(non_camel_case_types)]
// // #[derive(Debug, Serialize)]
// // struct Status {
// //     code: i16,
// //     message: String,
// // }

// // #[allow(non_camel_case_types)]
// // #[derive(Debug, Serialize)]
// // struct AdmissionResponse {
// //     uid: Value,
// //     allowed: bool,
// //     status: Status,
// // }

// // #[derive(Debug, PartialEq, Eq, Hash, EncodeLabelSet, Clone)]
// // struct Labels {
// //     method: Method,
// //     response: String,
// // }

// // #[derive(Debug, PartialEq, Eq, Hash, EncodeLabelValue, Clone)]
// // enum Method {
// //     GET,
// //     POST,
// // }

// // #[allow(non_snake_case)]
// // #[derive(Debug, Serialize)]
// // struct AdmissionReview {
// //     apiVersion: String,
// //     kind: String,
// //     response: AdmissionResponse,
// // }

// // #[derive(Debug, Clone)]
// // struct Decider {
// //     state: bool,
// // }

// // #[derive(Debug, Clone)]
// // struct Metrics {
// //     counter: Family<Labels, Counter>,
// // }

// // #[derive(Debug)]
// // struct Register {
// //     registry: Registry,
// // }

// // // async fn schema_validation_middleware(req: Request, next: Next) -> impl IntoResponse {
// // //     let (parts, body) = req.into_parts();
// // //     let buffer = match to_bytes(body, 5_000_000).await {
// // //         Ok(bytes) => bytes,
// // //         Err(_err) => return StatusCode::BAD_REQUEST.into_response(),
// // //     };

// // //     let data = serde_json::from_slice::<Value>(&buffer).unwrap();

// // //     let uid = &data["request"]["uid"];
// // //     println!("uid is : {}", uid);

// // //     let constructed_request = Request::from_parts(parts, Body::from(buffer));
// // //     let resp = next.run(constructed_request).await;
// // //     resp
// // // }

// // // middleware for pormetheus metrics which parses the outgoing response and increments counter based on the response
// // async fn metric_middleware(
// //     State(metrics): State<Arc<Mutex<Metrics>>>,
// //     req: Request,
// //     next: Next,
// // ) -> Response<Body> {
// //     let resp = next.run(req).await;

// //     let result = match &resp.extensions().get::<Decider>() {
// //         Some(val) if val.state == true => true,
// //         Some(_) => false,
// //         None => false,
// //     };

// //     if result == true {
// //         metrics
// //             .lock()
// //             .await
// //             .counter
// //             .get_or_create(&Labels {
// //                 method: Method::POST,
// //                 response: "allowed".to_string(),
// //             })
// //             .inc();
// //         return resp;
// //     } else {
// //         metrics
// //             .lock()
// //             .await
// //             .counter
// //             .get_or_create(&Labels {
// //                 method: Method::POST,
// //                 response: "denied".to_string(),
// //             })
// //             .inc();
// //         return resp;
// //     }
// // }

// // // set k8s OpneAPI enabled version as environment varaible `K8S_OPENAPI_ENABLED_VERSION=1.50`
// // #[tokio::main]
// // async fn main() {
// //     env_logger::init();

// //     // the watcher to check for tags resource with status field set to active
// //     // if it is not set to active don't cache its tags
// //     let client = kube::Client::try_default().await.unwrap();

// //     let tags: Api<Tag> = Api::all(client);

// //     let wc = watcher::Config::default();

// //     let cache_state = ImageCache {
// //         cache: ImageCache::default(),
// //     };

// //     // let cache_images = Arc::new(Mutex::new(cache_state));
// //     let cache_images = Arc::new(RwLock::new(cache_state));
// //     let cache = cache_images.clone();

// //     let mut events = watcher(tags, wc).applied_objects().boxed();

// //     let _ = tokio::spawn(async move {
// //         loop {
// //             while let Some(event) = events.try_next().await.unwrap() {
// //                 // let cache = Arc::clone(&cache_images);
// //                 // let _ = cache
// //                 //     .lock()
// //                 //     .await
// //                 //     .cache
// //                 //     .insert(event.spec.image.to_string(), "vulnerable".to_string())
// //                 //     .await;
// //                 let resp = cache.write().map(|s| {
// //                     s.cache.insert(event.spec.image.to_string(), "vulnerable".to_string());
// //                     log::info!(
// //                         "events : {:?} cache : {:?}",
// //                         event.metadata.name.unwrap_or_else(|| "no name".to_string()),
// //                         cache.read().unwrap_err()
// //                     )
// //                 }).map_err(|s| {
// //                     log::info!("error occured while getting the RwLock Guard");
// //                     Err(s)
// //                 });
// //             }
// //         }
// //     });

// //     let mut state = Register {
// //         registry: Registry::default(),
// //     };

// //     let metrics = Metrics {
// //         counter: Family::default(),
// //     };

// //     state.registry.register(
// //         "image_policy_admission_requests",
// //         "number of admission requests",
// //         metrics.counter.clone(),
// //     );

// //     let metrics_state = Arc::new(Mutex::new(metrics));
// //     let registry_state = Arc::new(Mutex::new(state));

// //     // router
// //     let app = Router::new()
// //         .route(
// //             "/",
// //             get(|| async {
// //                 info!("got request");
// //                 "Hello World!"
// //             }),
// //         )
// //         .route("/validate", post(validate))
// //         .with_state(cache_images.clone())
// //         .layer(middleware::from_fn_with_state(
// //             metrics_state.clone(),
// //             metric_middleware,
// //         ))
// //         .route("/metrics", get(metrics_handler).with_state(registry_state));

// //     let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

// //     info!("starting the server at {:?}", listener.local_addr());
// //     axum::serve(listener, app).await.unwrap();
// // }

// // async fn validate(
// //     State(cache): State<Arc<Mutex<ImageCache>>>,
// //     payload: Json<Value>,
// // ) -> impl IntoResponse {
// //     let resp = |allow: bool, uid: Value, code: i16, message: String| AdmissionReview {
// //         apiVersion: "admission.k8s.io/v1".to_string(),
// //         kind: "AdmissionReview".to_string(),
// //         response: AdmissionResponse {
// //             uid: (uid),
// //             allowed: (allow),
// //             status: Status {
// //                 code: (code),
// //                 message: (message),
// //             },
// //         },
// //     };

// //     let image = payload["request"]["image"].to_string();
// //     log::info!("image name : {}", &image);
// //     let image_value = cache
// //         .clone()
// //         .lock()
// //         .await
// //         .cache
// //         .get(image.as_str())
// //         .await
// //         .unwrap_or_else(|| "not found".to_string());

// //     log::info!(
// //         "image : {image_value}, cache : {:?}",
// //         cache.clone().lock().await.cache
// //     );

// //     let uid = payload["request"]["uid"].clone();

// //     if payload["request"]["allow"] == "true".to_string() {
// //         info!("valid request");
// //         Response::builder()
// //             .header("Content-Type", "application/json")
// //             .extension(Decider { state: true })
// //             .body(Body::from(
// //                 serde_json::to_vec(&resp(true, uid, 200, "valid request".to_string())).unwrap(),
// //             ))
// //             .unwrap()
// //     } else {
// //         warn!(
// //             "Invalid request by {}",
// //             &payload["request"]["userInfo"]["username"]
// //         );
// //         Response::builder()
// //             .header("Content-Type", "application/json")
// //             .extension(Decider { state: false })
// //             .body(Body::from(
// //                 serde_json::to_vec(&resp(false, uid, 403, "Invalid Request!".to_string())).unwrap(),
// //             ))
// //             .unwrap()
// //     }
// // }

// // async fn metrics_handler(State(metrics): State<Arc<Mutex<Register>>>) -> impl IntoResponse {
// //     let state = metrics.lock().await;
// //     let mut buffer = String::new();
// //     encode(&mut buffer, &state.registry).unwrap();
// //     buffer
// // }


// use axum::{
//     Extension, Json, Router,
//     body::{Body, Bytes, to_bytes},
//     extract::{Request, State},
//     http::{Error, Response, StatusCode},
//     middleware::{self, Next},
//     response::IntoResponse,
//     routing::{get, post},
// };
// use crd::crd::{Tag, TagSpec, TagStatus};
// use futures::{StreamExt, TryStreamExt, lock::Mutex};
// use k8s_openapi::{ByteString, api::core::v1::Pod};
// use kube::{
//     Api, CustomResource, ResourceExt,
//     api::{ListParams, WatchEvent},
//     core::{
//         admission::{self, AdmissionRequest},
//         response::StatusCause,
//     },
//     runtime::{
//         WatchStreamExt,
//         watcher::{self, Event, watcher},
//     },
// };
// mod cache;
// use crate::cache::cache::ImageCache;
// use log::{info, warn};
// use moka::future::Cache;
// use prometheus_client::{
//     encoding::{DescriptorEncoder, EncodeLabelSet, EncodeLabelValue, text::encode},
//     metrics::{counter::Counter, family::Family},
//     registry::{self, Registry},
// };
// use schemars::JsonSchema;
// use serde::{Deserialize, Serialize};
// use serde_json::{Value, to_string};
// use std::{collections::HashSet, fmt::Display, sync::Arc};
// use tokio::{net::TcpListener, sync::RwLock};
// use tower::{ServiceBuilder, buffer};

// #[allow(non_camel_case_types)]
// #[derive(Debug, Serialize)]
// struct Status {
//     code: i16,
//     message: String,
// }

// #[allow(non_camel_case_types)]
// #[derive(Debug, Serialize)]
// struct AdmissionResponse {
//     uid: Value,
//     allowed: bool,
//     status: Status,
// }

// #[derive(Debug, PartialEq, Eq, Hash, EncodeLabelSet, Clone)]
// struct Labels {
//     method: Method,
//     response: String,
// }

// #[derive(Debug, PartialEq, Eq, Hash, EncodeLabelValue, Clone)]
// enum Method {
//     GET,
//     POST,
// }

// #[allow(non_snake_case)]
// #[derive(Debug, Serialize)]
// struct AdmissionReview {
//     apiVersion: String,
//     kind: String,
//     response: AdmissionResponse,
// }

// #[derive(Debug, Clone)]
// struct Decider {
//     state: bool,
// }

// #[derive(Debug, Clone)]
// struct Metrics {
//     counter: Family<Labels, Counter>,
// }

// #[derive(Debug)]
// struct Register {
//     registry: Registry,
// }

// // async fn schema_validation_middleware(req: Request, next: Next) -> impl IntoResponse {
// //     let (parts, body) = req.into_parts();
// //     let buffer = match to_bytes(body, 5_000_000).await {
// //         Ok(bytes) => bytes,
// //         Err(_err) => return StatusCode::BAD_REQUEST.into_response(),
// //     };

// //     let data = serde_json::from_slice::<Value>(&buffer).unwrap();

// //     let uid = &data["request"]["uid"];
// //     println!("uid is : {}", uid);

// //     let constructed_request = Request::from_parts(parts, Body::from(buffer));
// //     let resp = next.run(constructed_request).await;
// //     resp
// // }

// // middleware for pormetheus metrics which parses the outgoing response and increments counter based on the response
// async fn metric_middleware(
//     State(metrics): State<Arc<Mutex<Metrics>>>,
//     req: Request,
//     next: Next,
// ) -> Response<Body> {
//     let resp = next.run(req).await;

//     let result = match &resp.extensions().get::<Decider>() {
//         Some(val) if val.state == true => true,
//         Some(_) => false,
//         None => false,
//     };

//     if result == true {
//         metrics
//             .lock()
//             .await
//             .counter
//             .get_or_create(&Labels {
//                 method: Method::POST,
//                 response: "allowed".to_string(),
//             })
//             .inc();
//         return resp;
//     } else {
//         metrics
//             .lock()
//             .await
//             .counter
//             .get_or_create(&Labels {
//                 method: Method::POST,
//                 response: "denied".to_string(),
//             })
//             .inc();
//         return resp;
//     }
// }

// // set k8s OpneAPI enabled version as environment varaible `K8S_OPENAPI_ENABLED_VERSION=1.50`
// #[tokio::main]
// async fn main() {
//     env_logger::init();

//     // the watcher to check for tags resource with status field set to active
//     // if it is not set to active don't cache its tags
//     let client = kube::Client::try_default().await.unwrap();

//     let tags: Api<Tag> = Api::all(client);

//     let wc = watcher::Config::default();

//     let cache_state = ImageCache {
//         cache: ImageCache::default(),
//     };

//     // let cache_images = Arc::new(Mutex::new(cache_state));
//     let cache_images = Arc::new(RwLock::new(cache_state));
//     let cache = cache_images.clone();

//     let mut events = watcher(tags, wc).applied_objects().boxed();

//     let _ = tokio::spawn(async move {
//         loop {
//             while let Some(event) = events.try_next().await.unwrap() {
//                 let guard = cache.write().await;
//                 guard
//                     .cache
//                     .insert(event.spec.image.to_string(), "vulnerable".to_string())
//                     .await;
//                 log::info!(
//                     "events : {:?} cache : {:?}",
//                     event.metadata.name.unwrap_or_else(|| "no name".to_string()),
//                     guard.cache
//                 );
//             }
//         }
//     });

//     let mut state = Register {
//         registry: Registry::default(),
//     };

//     let metrics = Metrics {
//         counter: Family::default(),
//     };

//     state.registry.register(
//         "image_policy_admission_requests",
//         "number of admission requests",
//         metrics.counter.clone(),
//     );

//     let metrics_state = Arc::new(Mutex::new(metrics));
//     let registry_state = Arc::new(Mutex::new(state));

//     // router
//     let app = Router::new()
//         .route(
//             "/",
//             get(|| async {
//                 info!("got request");
//                 "Hello World!"
//             }),
//         )
//         .route("/validate", post(validate))
//         .with_state(cache_images.clone())
//         .layer(middleware::from_fn_with_state(
//             metrics_state.clone(),
//             metric_middleware,
//         ))
//         .route("/metrics", get(metrics_handler).with_state(registry_state));

//     let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

//     info!("starting the server at {:?}", listener.local_addr());
//     axum::serve(listener, app).await.unwrap();
// }

// async fn validate(
//     State(cache): State<Arc<RwLock<ImageCache>>>,
//     payload: Json<Value>,
// ) -> impl IntoResponse {
//     let resp = |allow: bool, uid: Value, code: i16, message: String| AdmissionReview {
//         apiVersion: "admission.k8s.io/v1".to_string(),
//         kind: "AdmissionReview".to_string(),
//         response: AdmissionResponse {
//             uid: (uid),
//             allowed: (allow),
//             status: Status {
//                 code: (code),
//                 message: (message),
//             },
//         },
//     };

//     let image = payload["request"]["image"].as_str().unwrap_or("what");
//     log::info!("image name : {}", &image);

//     let uid = payload["request"]["uid"].clone();

//     let image_value = cache.read().await.cache.get(image).await;

//     if image_value.is_some() {
//         Response::builder()
//             .header("Content-Type", "application/json")
//             .extension(Decider { state: true })
//             .body(Body::from(serde_json::to_vec(&resp(
//                 true,
//                 uid,
//                 200,
//                 "valid request".to_string(),
//             ))))
//             .unwrap();
//     } else {
//         Response::builder()
//             .header("Content-Type", "application/json")
//             .extension(Decider { state: false })
//             .body(Body::from(serde_json::to_vec(&resp(
//                 false,
//                 uid,
//                 200,
//                 "valid request".to_string(),
//             ))))
//             .unwrap();
//     }

//     if let Some(image_value) = cache.read().await.cache.get(image).await {
//         if !image_value.is_empty() {
//             info!("valid request");
//             Response::builder()
//                 .header("Content-Type", "application/json")
//                 .extension(Decider { state: true })
//                 .body(Body::from(
//                     serde_json::to_vec(&resp(false, uid, 200, "valid request".to_string()))
//                         .unwrap(),
//                 ))
//                 .unwrap();
//         } else {
//             Response::builder()
//                 .header("Content-Type", "application/json")
//                 .extension(Decider { state: true })
//                 .body(Body::from(
//                     serde_json::to_vec(&resp(true, uid, 200, "valid request".to_string())).unwrap(),
//                 ))
//                 .unwrap();
//         }
//     }

//     if payload["request"]["allow"] == "true".to_string() {
//         info!("valid request");
//         Response::builder()
//             .header("Content-Type", "application/json")
//             .extension(Decider { state: true })
//             .body(Body::from(
//                 serde_json::to_vec(&resp(true, uid, 200, "valid request".to_string())).unwrap(),
//             ))
//             .unwrap()
//     } else {
//         warn!(
//             "Invalid request by {}",
//             &payload["request"]["userInfo"]["username"]
//         );
//         Response::builder()
//             .header("Content-Type", "application/json")
//             .extension(Decider { state: false })
//             .body(Body::from(
//                 serde_json::to_vec(&resp(false, uid, 403, "Invalid Request!".to_string())).unwrap(),
//             ))
//             .unwrap()
//     }
// }

// async fn metrics_handler(State(metrics): State<Arc<Mutex<Register>>>) -> impl IntoResponse {
//     let state = metrics.lock().await;
//     let mut buffer = String::new();
//     encode(&mut buffer, &state.registry).unwrap();
//     buffer
// }