#let cv = yaml("cv.yaml")

#set document(title: cv.personal.name + " - CV", author: cv.personal.name)
#set page(margin: (x: 1.8cm, y: 1.4cm))
#set text(font: "New Computer Modern", size: 10pt)
#set par(justify: false, leading: 0.55em)
#set list(indent: 4pt, body-indent: 5pt, spacing: 0.5em)

// Colors
#let accent = rgb("#f74c00")
#let dark = rgb("#2a3439")

#let strip-scheme(url) = url.replace("https://", "").replace("http://", "")

#let section(title) = {
  v(8pt)
  text(size: 12pt, weight: "bold", fill: accent)[#title]
  v(-6pt)
  line(length: 100%, stroke: 0.5pt + accent)
  v(2pt)
}

// Header
#align(center)[
  #text(size: 22pt, weight: "bold", fill: dark)[#cv.personal.name]
  #v(2pt)
  #text(size: 12pt, fill: accent)[#cv.personal.title]
  #v(2pt)
  #text(size: 9.5pt)[
    #cv.personal.location · #cv.work_preferences.employment, #lower(cv.work_preferences.arrangement)
  ]
  #v(2pt)
  #text(size: 9pt)[
    #link("mailto:" + cv.personal.email)[#cv.personal.email] ·
    #link(cv.personal.linkedin)[#strip-scheme(cv.personal.linkedin)] ·
    #link(cv.personal.github)[#strip-scheme(cv.personal.github)] ·
    #link(cv.personal.homepage)[#strip-scheme(cv.personal.homepage)]
  ]
]

#section[Summary]
#cv.personal.intro.split("\n").filter(l => l.trim() != "").join(" ")

#section[Skills]
#grid(
  columns: (auto, 1fr),
  row-gutter: 5pt,
  column-gutter: 12pt,
  ..cv.skills.map(s => (text(weight: "bold")[#s.category], s.items.join(", "))).flatten()
)

#section[Experience]
#for job in cv.experience [
  #block(breakable: true)[
    #grid(
      columns: (1fr, auto),
      [#text(weight: "bold")[#job.title] #text(fill: dark.lighten(30%))[·] #job.company],
      text(style: "italic", size: 9pt)[#job.date]
    )
    #v(-2pt)
    #list(..job.highlights.map(h => [#h]))
    #for proj in job.at("projects", default: ()) [
      #v(1pt)
      #block(breakable: false, pad(left: 10pt)[
        #text(weight: "bold", style: "italic")[#proj.name]
        #v(-4pt)
        #list(..proj.highlights.map(h => [#h]))
      ])
    ]
    #v(-2pt)
    #text(size: 8.5pt, fill: dark.lighten(30%))[Tech: #job.skills.join(", ")]
  ]
  #v(6pt)
]

#section[Education]
#for edu in cv.education [
  #grid(
    columns: (1fr, auto),
    [#text(weight: "bold")[#edu.degree] · #edu.institution],
    text(style: "italic", size: 9pt)[#edu.date]
  )
]

#section[Languages]
#cv.languages.map(l => l.name + " (" + l.level + ")").join(", ")

#section[Interests]
#list(..cv.interests.map(i => [#text(weight: "bold")[#i.name] --- #i.description]))
