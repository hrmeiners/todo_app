// explicitly importing rocket so all of its macros are installed globally
#[macro_use] extern crate rocket;

use rocket::{serde::{Deserialize, json::Json, Serialize}, response::{Responder, self}, http::Status, Request};
use rocket_db_pools::{Database, Connection};


#[derive(Deserialize, Serialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
struct Task {
    id: i64,
    item: String
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct TaskItem<'r> {
    item: &'r str
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct TaskId {
    id: i64
}


#[derive(Database)]
#[database("todo")]
struct TodoDatabase(sqlx::PgPool);

struct DatabaseError(rocket_db_pools::sqlx::Error);

impl<'r> Responder<'r, 'r> for DatabaseError {
    fn respond_to(self, request: &Request) -> response::Result<'r> {
        Err(Status::InternalServerError)
    }
}

impl From<rocket_db_pools::sqlx::Error> for DatabaseError {
    fn from(error: rocket_db_pools::sqlx::Error) -> Self {
        DatabaseError(error)
    }
}


/*  add_task notes
    - post("/addtask"): whenever an HTTP post request is called with
    path "/addtask", the add_task function will be called
    - data="<task>": body data should enter the task parameter which
    then gets parsed as a Json and saved into a Task struct
*/
#[post("/addtask", data="<task>")]
async fn add_task(task: Json<TaskItem<'_>>, mut db: Connection<TodoDatabase>) -> Result<Json<Task>, DatabaseError> {
    let added_task = sqlx::query_as::<_, Task>("INSERT INTO tasks (item) VALUES ($1) RETURNING *")
        .bind(task.item)
        .fetch_one(&mut *db)
        .await?;

    Ok(Json(added_task))
}

/*
    read_tasks notes
    - get("/readtasks"): called when rocket recieves an HHTP get request 
    with path "/readtasks"
*/
#[get("/readtasks")]
async fn read_tasks(mut db: Connection<TodoDatabase>) -> Result<Json<Vec<Task>>, DatabaseError> {
    let all_tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
        .fetch_all(&mut *db)
        .await?;
        
    Ok(Json(all_tasks))
}


/*
    edit_task notes
    - body of HHTP request needs {"id": __, "item": __ }
*/
#[put("/edittask", data="<task_update>")]
async fn edit_task(task_update: Json<Task>, mut db: Connection<TodoDatabase>) -> Result<Json<Task>, DatabaseError> {
    let updated_task = sqlx::query_as::<_, Task>("UPDATE tasks SET item = $1 WHERE id = $2 RETURNING *")
    .bind(&task_update.item)
    .bind(task_update.id)
    .fetch_one(&mut *db)
    .await?;

    Ok(Json(updated_task))
}

/*
    delete_task notes
    - body of HHTP request needs {"id": __ }
*/
#[delete("/deletetask", data="<task_id>")]
async fn delete_task(task_id: Json<TaskId>, mut db: Connection<TodoDatabase>) -> Result<Json<Task>, DatabaseError> {
    let deleted_task = sqlx::query_as::<_, Task>("DELETE FROM tasks WHERE id = $1 RETURNING *")
        .bind(task_id.id)
        .fetch_one(&mut *db)
        .await?;
        
    Ok(Json(deleted_task))
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


/*
    rocket notes
    - #[launch]: when the code is run, rocket will be called first
    - launch does some other stuff too but idk yet
*/
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(TodoDatabase::init())
        .mount("/", routes![index, add_task, read_tasks, edit_task, delete_task])
}


/*
    Following along with tutorial:
    https://betterprogramming.pub/how-to-write-a-web-app-in-rust-part-2-2da195369fc1 

*/