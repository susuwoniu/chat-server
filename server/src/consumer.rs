use crate::{
    global::Config, im::service::send_welcome::welcome, middleware::Locale, notification::error,
};
use chrono::Utc;
use queue_file::QueueFile;
use std::sync::Arc;
use std::time::Duration;
use tokio::{sync::Mutex, time::sleep};
pub async fn consumer(qf_mutex: Arc<Mutex<QueueFile>>) {
    // let mut interval = time::interval(time::Duration::from_secs(2));
    let cfg = Config::global();
    loop {
        // interval.tick().await;
        let now = Utc::now();
        // tracing::debug!("consumer tick {:?}", now);
        let item = qf_mutex.lock().await.peek().unwrap();
        if let Some(item) = item {
            let item_str = std::str::from_utf8(&item).unwrap_or("");

            if item_str.len() > 0 {
                tracing::info!("consumer item: {}", item_str);
                let split = item_str.split("::");
                let vec = split.collect::<Vec<&str>>();
                if vec.len() == 3 {
                    let kind = vec[0];

                    if kind == "signup" {
                        let locale = Locale::new(vec[1]);
                        let target_account_id = vec[2].parse::<i64>().unwrap();
                        // send welcome message
                        //
                        let result = welcome(&locale, target_account_id).await;
                        if let Err(e) = result {
                            tracing::error!("consumer error: {}", e);
                            let _ = error("signup消费者错误", format!("{}", e)).await;
                            let mut qf = qf_mutex.lock().await;
                            qf.remove().unwrap();
                            // qf.add(item_str.as_bytes()).unwrap();
                            sleep(Duration::from_secs(cfg.consumer_duration_in_seconds)).await;
                        } else {
                            tracing::info!("success consumer item {}", item_str);
                            let mut qf = qf_mutex.lock().await;
                            qf.remove().unwrap();
                        }
                        continue;
                    }
                }
                qf_mutex.lock().await.remove().unwrap();
                tracing::error!("can not parse item: {}", item_str);
                let _ = error(
                    "signup消费者错误",
                    format!("can not parse item: {}", item_str),
                )
                .await;

                sleep(Duration::from_secs(cfg.consumer_duration_in_seconds)).await;
            } else {
                // invalid
                tracing::error!("invalid queue item: {}", item_str);
                let _ = error(
                    "signup消费者错误",
                    format!("invalid queue item: {}", item_str),
                )
                .await;
                qf_mutex.lock().await.remove().unwrap();

                sleep(Duration::from_secs(cfg.consumer_duration_in_seconds)).await;
            }
        } else {
            sleep(Duration::from_secs(cfg.consumer_duration_in_seconds)).await;
        }
        // consumer(qf).await;
        // println!("consumer tick end",);
    }

    // Box::pin(async move {
    //     consumer(qf).await;
    // })
}
