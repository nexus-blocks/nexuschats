use ic_cdk::query;
use crate::PROJECTS;

use crate::common::types::Project;

#[query]
fn get_my_projects() -> Vec<Project> {
    let caller = ic_cdk::caller();
    PROJECTS.with(|s| {
        s.borrow().iter().filter(|(_, v)| v.user_id == caller.to_text()).map(|(_, v)| v.clone()).collect()
    })
}

#[query]
fn get_project(id: String) -> Result<Project, String> {
    PROJECTS.with(|s| {
        match s.borrow().get(&id) {
            Some(project) => Ok(project.clone()),
            None => Err(format!("project with id={} not found", id)),
        }
    })
}

#[query]
fn get_all_projects() -> Vec<Project> {
    PROJECTS.with(|s| s.borrow().iter().map(|(_, v)| v.clone()).collect())
}

