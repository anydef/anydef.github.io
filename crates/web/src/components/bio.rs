use leptos::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Cv {
    personal: Personal,
    experience: Vec<Experience>,
    education: Vec<Education>,
    skills: Vec<SkillCategory>,
    interests: Vec<Interest>,
}

#[derive(Deserialize)]
struct Personal {
    name: String,
    title: String,
    location: String,
    intro: String,
    email: String,
    github: String,
    linkedin: String,
}

#[derive(Deserialize, Clone)]
struct Experience {
    title: String,
    company: String,
    date: String,
    skills: Vec<String>,
    highlights: Vec<String>,
    #[serde(default)]
    projects: Vec<Project>,
}

#[derive(Deserialize, Clone)]
struct Project {
    name: String,
    highlights: Vec<String>,
}

#[derive(Deserialize, Clone)]
struct Education {
    degree: String,
    institution: String,
    date: String,
}

#[derive(Deserialize, Clone)]
struct SkillCategory {
    category: String,
    items: Vec<String>,
}

#[derive(Deserialize, Clone)]
struct Interest {
    name: String,
    description: String,
}

fn load_cv() -> Cv {
    const CV_YAML: &str = include_str!("../../../../cv.yaml");
    serde_yaml::from_str(CV_YAML).expect("Failed to parse cv.yaml")
}

#[component]
pub fn Bio() -> impl IntoView {
    let cv = load_cv();

    view! {
        <section class="bio">
            <div class="bio-content">
                <div class="bio-header">
                    <h2>"About Me"</h2>
                </div>

                <div class="bio-section">
                    <p class="bio-intro">{&cv.personal.intro}</p>
                </div>

                <div class="bio-section">
                    <h3>"Experience"</h3>
                    <For
                        each=move || cv.experience.clone()
                        key=|exp| format!("{}-{}", exp.company, exp.date)
                        children=move |exp| {
                            let skills = exp.skills.clone();
                            let highlights = exp.highlights.clone();
                            let projects = exp.projects.clone();
                            view! {
                                <div class="bio-item">
                                    <div class="bio-item-header">
                                        <span class="bio-item-title">{&exp.title}</span>
                                        <span class="bio-item-date">{&exp.date}</span>
                                    </div>
                                    <span class="bio-item-company">{&exp.company}</span>
                                    <ul class="bio-item-highlights">
                                        {highlights.into_iter().map(|h| view! { <li>{h}</li> }).collect_view()}
                                    </ul>
                                    {projects.into_iter().map(|p| view! {
                                        <div class="bio-item-project">
                                            <span class="bio-item-project-name">{p.name}</span>
                                            <ul class="bio-item-highlights">
                                                {p.highlights.into_iter().map(|h| view! { <li>{h}</li> }).collect_view()}
                                            </ul>
                                        </div>
                                    }).collect_view()}
                                    <div class="bio-item-skills">
                                        <For
                                            each=move || skills.clone()
                                            key=|skill| skill.clone()
                                            children=move |skill| view! {
                                                <span class="bio-item-skill">{skill}</span>
                                            }
                                        />
                                    </div>
                                </div>
                            }
                        }
                    />
                </div>

                <div class="bio-section">
                    <h3>"Skills"</h3>
                    <For
                        each=move || cv.skills.clone()
                        key=|cat| cat.category.clone()
                        children=move |cat| view! {
                            <div class="bio-skills-category">
                                <h4>{cat.category.clone()}</h4>
                                <div class="bio-skills">
                                    <For
                                        each=move || cat.items.clone()
                                        key=|skill| skill.clone()
                                        children=move |skill| view! {
                                            <span class="bio-skill">{skill}</span>
                                        }
                                    />
                                </div>
                            </div>
                        }
                    />
                </div>

                <div class="bio-section">
                    <h3>"Education"</h3>
                    <For
                        each=move || cv.education.clone()
                        key=|edu| edu.degree.clone()
                        children=move |edu| view! {
                            <div class="bio-item">
                                <div class="bio-item-header">
                                    <span class="bio-item-title">{&edu.degree}</span>
                                    <span class="bio-item-date">{&edu.date}</span>
                                </div>
                                <span class="bio-item-company">{&edu.institution}</span>
                            </div>
                        }
                    />
                </div>

                <div class="bio-section">
                    <h3>"Interests"</h3>
                    <div class="bio-interests">
                        <For
                            each=move || cv.interests.clone()
                            key=|interest| interest.name.clone()
                            children=move |interest| view! {
                                <div class="bio-interest">
                                    <span class="bio-interest-name">{&interest.name}</span>
                                    <p class="bio-interest-desc">{&interest.description}</p>
                                </div>
                            }
                        />
                    </div>
                </div>

                <div class="bio-section">
                    <h3>"Contact"</h3>
                    <div class="bio-contact">
                        <span class="bio-location">{&cv.personal.location}</span>
                        <a href={format!("mailto:{}", cv.personal.email)}>{&cv.personal.email}</a>
                        <a href={cv.personal.github.clone()} target="_blank" rel="noopener noreferrer">"GitHub"</a>
                        <a href={cv.personal.linkedin.clone()} target="_blank" rel="noopener noreferrer">"LinkedIn"</a>
                    </div>
                </div>
            </div>
        </section>
    }
}
