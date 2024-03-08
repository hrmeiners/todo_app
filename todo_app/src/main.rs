// explicitly importing rocket so all of its macros are installed globally
#[macro_use] extern crate rocket;


mod pool;


use migration::MigratorTrait;
use pool::Db;
use rocket::{fairing::{AdHoc, self}, Rocket, Build, form::Form, serde::json::Json, http::Status, response::{Responder, self}, Request};
use sea_orm::{ActiveModelTrait, Set, EntityTrait, QueryOrder, DeleteResult};
use sea_orm_rocket::{Database, Connection};

use entity::tasks;
use entity::tasks::Entity as Tasks;


struct DatabaseError(sea_orm::DbErr);

impl<'r> Responder<'r, 'r> for DatabaseError {
    fn respond_to(self, _request: &Request) -> response::Result<'r> {
        Err(Status::InternalServerError)
    }
}

impl From<sea_orm::DbErr> for DatabaseError {
    fn from(error: sea_orm::DbErr) -> Self {
        DatabaseError(error)
    }
}

#[post("/addtask", data="<task_form>")]
async fn add_task(conn: Connection<'_, Db>, task_form: Form<tasks::Model>) -> Result<Json<tasks::Model>, DatabaseError> {
    let db = conn.into_inner();
    let task = task_form.into_inner();

    let active_task: tasks::ActiveModel = tasks::ActiveModel {
        item: Set(task.item),
        ..Default::default()
    };

    Ok(Json(active_task.insert(db).await?))
}

#[get("/readtasks")]
async fn read_tasks(conn: Connection<'_, Db>) -> Result<Json<Vec<tasks::Model>>, DatabaseError> {
    let db = conn.into_inner();

    Ok(Json(
        Tasks::find()
            .order_by_asc(tasks::Column::Id)
            .all(db)
            .await?
    ))
}

#[put("/edittask", data="<task_form>")]
async fn edit_task(conn: Connection<'_, Db>, task_form: Form<tasks::Model>) -> Result<Json<tasks::Model>, DatabaseError> {
    let db = conn.into_inner();
    let task = task_form.into_inner();

    let task_to_update = Tasks::find_by_id(task.id).one(db).await?;
    let mut task_to_update: tasks::ActiveModel = task_to_update.unwrap().into();

    Ok(Json(
        task_to_update.update(db).await?
    ))
}

#[delete("/deletetask/<id>")]
async fn delete_task(conn: Connection<'_, Db>, id: i32) -> Result<String, DatabaseError> {
    let db = conn.into_inner();
    let result = Tasks::delete_by_id(id).exec(db).await?;

    Ok(format!("{} task(s) deleted", result.rows_affected))
}


/*
    index notes
    - get("/"): whenever an HTTP get request is called with path "/",
    the index function will be called
*/  
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;

    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}


/*
    rocket notes
    - #[launch]: when the code is run, rocket will be called first
    - launch does some other stuff too but idk yet
*/
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/", routes![index, add_task, read_tasks, edit_task, delete_task])
}


/*
    Following along with tutorial:
    https://betterprogramming.pub/how-to-write-a-web-app-in-rust-part-2-2da195369fc1 

*/