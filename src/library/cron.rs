use tokio::runtime;
// handles cronjobs in the app
use crate::app::mongo::AppInstance;
use tokio_cron::{daily, Job, Scheduler};

pub fn init_cron_job(app: AppInstance) {
    let app0 = app.clone();
    let app1 = app.clone();
    let app2 = app.clone();
    let app3 = app.clone();
    let app4 = app.clone();

    // crontab format (6 stars): sec-> * min-> * hour-> * day-> * month-> * day-of-week-> *

    // You can use a local (timezone) scheduler, or a UTC scheduler.
    let mut scheduler = Scheduler::local();

    // runs every 1 hour
    scheduler.add(Job::new("0 0 * * * *", move || {
        let app = app0.clone();
        async move { hourly_cron_job(app).await }
    }));

    // runs @ 00:00 every day
    scheduler.add(Job::named("daily cron job", daily("0"), move || {
        let app1 = app1.clone();
        daily_cron_job(app1)
    }));

    // runs every week @ 00:00
    scheduler.add(Job::new("0 0 0 */7 * *", move || {
        let app2 = app2.clone();
        async move { weekly_cron_job(app2).await }
    }));

    // runs on the first day of every month @ 00:00
    scheduler.add(Job::new("0 0 0 1 * *", move || {
        let app3 = app3.clone();
        async move { monthly_cron_job(app3).await }
    }));

    // runs on the first day of every quarter (i.e every 3 months) @ 00:00
    scheduler.add(Job::new("0 0 0 1 */3 *", move || {
        let app4 = app4.clone();
        async move { quarterly_cron_job(app4).await }
    }));
}

async fn hourly_cron_job(app: AppInstance) {
    // let app2 = app.clone();
    // let closure = move ||{
    //     let app2 = app2;
    //     let async_block = async{

    //         // execute unpublished broadcasts
    //         let _ = broadcast::execute_unpublished_broadcasts(&app2).await;
    //     };
    //     let rt = runtime::Runtime::new().unwrap();
    //     rt.block_on(async_block);
    // };
    // app.pool.run_job(closure);
}

async fn daily_cron_job(_app: AppInstance) {}

async fn weekly_cron_job(_app: AppInstance) {}

async fn monthly_cron_job(_app: AppInstance) {}

async fn quarterly_cron_job(_app: AppInstance) {}
