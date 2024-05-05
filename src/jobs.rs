use anyhow::anyhow;
use ndarray::array;
use serde::{Deserialize, Serialize};

use crate::{
    enums::{Language, Technology},
    utils::propagation,
};

#[derive(Serialize, Debug, Clone)]
pub struct Job {
    pub title: String,
    pub company: String,
    pub years_of_experience: i32,
    pub technologies: Vec<Technology>,
    pub languages: Vec<Language>,
    pub soft_skiills: i32,
}

#[derive(Serialize, Debug, Clone)]
pub enum JobCheckResult {
    Good,
    Bad,
    Maybe,
}

impl Job {
    pub fn get_score(&self, request: &JobCheckRequest) -> anyhow::Result<JobCheckResult> {
        let years_check = if request.years_of_experience >= self.years_of_experience {
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
            / self.languages.len() as f32;
        println!("{:?}", language_check);

        let soft_skills_check = match request.soft_skills.cmp(&self.soft_skiills) {
            std::cmp::Ordering::Greater => 1.0,
            std::cmp::Ordering::Equal => 1.0,
            std::cmp::Ordering::Less => request.soft_skills as f32 / self.soft_skiills as f32,
        };
        println!("{:?}", soft_skills_check);

        let results = propagation(array![
            years_check as f64 - 0.5,
            technology_check as f64 - 0.5,
            language_check as f64 - 0.5,
            soft_skills_check as f64 - 0.5,
        ]);
        let results = results.to_vec();
        let score = results
            .get(0)
            .ok_or_else(|| anyhow!("Results vec's length is 0"))?;
        if score <= &0.2 {
            return Ok(JobCheckResult::Bad);
        } else if score >= &0.8 {
            return Ok(JobCheckResult::Good);
        } else {
            return Ok(JobCheckResult::Maybe);
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct JobCheckRequest {
    pub years_of_experience: i32,
    pub technologies: Vec<Technology>,
    pub languages: Vec<Language>,
    pub soft_skills: i32,
}

pub fn get_jobs() -> Vec<Job> {
    vec![
        Job {
            title: "Software Engineer".to_string(),
            company: "Google".to_string(),
            years_of_experience: 5,
            technologies: vec![
                Technology::JavaScript,
                Technology::TypeScript,
                Technology::Go,
                Technology::Git,
                Technology::CPlusPlus,
                Technology::CSharp,
            ],
            languages: vec![Language::English, Language::Español],
            soft_skiills: 3,
        },
        Job {
            title: "Software Engineer".to_string(),
            company: "Facebook".to_string(),
            years_of_experience: 0,
            technologies: vec![Technology::JavaScript],
            languages: vec![Language::English, Language::Español],
            soft_skiills: 1,
        },
    ]
}
