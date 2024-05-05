use serde::{Deserialize, Serialize};

use crate::enums::{Language, Technology};

#[derive(Serialize, Debug, Clone)]
pub struct Job {
    pub title: String,
    pub company: String,
    pub years_of_experience: i32,
    pub technologies: Vec<Technology>,
    pub languages: Vec<Language>,
    pub soft_skiills: i32,
}

impl Job {
    pub fn get_score(&self, request: &JobCheckRequest) -> Vec<f32> {
        let years_check = if self.years_of_experience >= request.years_of_experience {
            1.0
        } else {
            0.0
        };
        println!("{:?}", years_check);

        // check how much percentage wise of the technologies of the job, the request has
        let technology_check = self
            .technologies
            .iter()
            .filter(|tech| request.technologies.contains(tech))
            .count() as f32
            / self.technologies.len() as f32;
        println!("{:?}", technology_check);

        let language_check = self
            .languages
            .iter()
            .filter(|lang| request.languages.contains(lang))
            .count() as f32
            / request.languages.len() as f32;
        println!("{:?}", language_check);

        let soft_skills_check = self.soft_skiills as f32 / request.soft_skiills as f32;
        println!("{:?}", soft_skills_check);

        vec![years_check, technology_check, language_check, soft_skills_check]
    }
}

#[derive(Deserialize, Debug)]
pub struct JobCheckRequest {
    pub years_of_experience: i32,
    pub technologies: Vec<Technology>,
    pub languages: Vec<Language>,
    pub soft_skiills: i32,
}

pub fn get_jobs() -> Vec<Job> {
    vec![Job {
        title: "Software Engineer".to_string(),
        company: "Google".to_string(),
        years_of_experience: 5,
        technologies: vec![
            Technology::JavaScript,
            Technology::TypeScript,
            Technology::Go,
            Technology::Git,
        ],
        languages: vec![Language::Engilsh],
        soft_skiills: 3,
    }]
}
