use uuid::Uuid;
use chrono::prelude::*;
use actix_web::{route, web, HttpResponse, Responder};
use crate::{
    model::{AppState, QueryOptions, Todo, UpdateTodoSchema},
    response::{GenericResponse, SingleTodoResponse, TodoData, TodoListResponse},
};

#[route("/todos", method = "GET")]
pub async fn get_todo(
    opts: web::Query<QueryOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let todos = data.todo_db.lock().unwrap();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let todos: Vec<Todo> = todos.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = TodoListResponse {
        status: "OK".to_string(),
        message: "Successfully retrieved todos".to_string(),
        results: todos.len(),
        todos,
    };

    HttpResponse::Ok().json(json_response)
}

#[route("/todos", method = "POST")]
async fn create_todo(
    mut body: web::Json<Todo>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut vec = data.todo_db.lock().unwrap();

    let todo = vec.iter().find(|todo| todo.title == body.title);

    if todo.is_some() {
        let error_response = GenericResponse {
            status: "ERROR".to_string(),
            message: format!("Todo with title: '{}' already exists", body.title),
            data: "".to_string(),
        };

        return HttpResponse::Conflict().json(error_response);
    }

    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now();

    body.id = Some(uuid_id.to_string());
    body.completed = Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    let todo = body.to_owned();

    vec.push(body.into_inner());

    let json_response = SingleTodoResponse {
        status: "OK".to_string(),
        message: "Successfully created todo".to_string(),
        data: TodoData { todo },
    };

    HttpResponse::Ok().json(json_response)
}

#[route("/todos/{id}", method = "GET")]
async fn find_todo(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let vec = data.todo_db.lock().unwrap();

    let id = path.into_inner();
    let todo = vec.iter().find(|todo| todo.id == Some(id.to_owned()));

    if todo.is_none() {
        let error_response = GenericResponse {
            status: "ERROR".to_string(),
            message: format!("Todo with ID: {} not found", id),
            data: "".to_string(),
        };

        return HttpResponse::NotFound().json(error_response);
    }

    let todo = todo.unwrap();
    let json_response = SingleTodoResponse {
        status: "success".to_string(),
        message: "Successfully retrieved todo".to_string(),
        data: TodoData { todo: todo.clone() },
    };

    HttpResponse::Ok().json(json_response)
}

#[route("/todos/{id}", method = "PUT")]
async fn update_todo(
    path: web::Path<String>,
    body: web::Json<UpdateTodoSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut vec = data.todo_db.lock().unwrap();

    let id = path.into_inner();
    let todo = vec.iter_mut().find(|todo| todo.id == Some(id.to_owned()));

    if todo.is_none() {
        let error_response = GenericResponse {
            status: "ERROR".to_string(),
            message: format!("Todo with ID: {} not found", id),
            data: "".to_string(),
        };

        return HttpResponse::NotFound().json(error_response);
    }

    let todo = todo.unwrap();
    let datetime = Utc::now();
    let title = body.title.to_owned().unwrap_or(todo.title.to_owned());
    let content = body.content.to_owned().unwrap_or(todo.content.to_owned());
    let payload = Todo {
        id: todo.id.to_owned(),
        title: if !title.is_empty() {
            title
        } else {
            todo.title.to_owned()
        },
        content: if !content.is_empty() {
            content
        } else {
            todo.content.to_owned()
        },
        completed: if body.completed.is_some() {
            body.completed
        } else {
            todo.completed
        },
        createdAt: todo.createdAt,
        updatedAt: Some(datetime),
    };
    *todo = payload;

    let json_response = SingleTodoResponse {
        status: "OK".to_string(),
        message: "Successfully updated todo".to_string(),
        data: TodoData { todo: todo.clone() },
    };

    HttpResponse::Ok().json(json_response)
}

#[route("/todos/{id}", method = "DELETE")]
async fn delete_todo(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let mut vec = data.todo_db.lock().unwrap();

    let id = path.into_inner();
    let todo = vec.iter_mut().find(|todo| todo.id == Some(id.to_owned()));

    if todo.is_none() {
        let error_response = GenericResponse {
            status: "ERROR".to_string(),
            message: format!("Todo with ID: {} not found", id),
            data: "".to_string(),
        };

        return HttpResponse::NotFound().json(error_response);
    }

    vec.retain(|todo| todo.id != Some(id.to_owned()));
    HttpResponse::NoContent().finish()
}