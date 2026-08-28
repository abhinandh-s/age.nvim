#import "@local/ui:0.1.0": *

#webpage("index.html", [age.nvim], pdf-path: "age-nvim.pdf")[
  #title()
  #outline(target: heading.where(bookmarked: false, outlined: true), depth: 2)
  
  #include "doc.typ"
  #include "license.typ"
] <home>

#document("age-nvim.pdf", title: [Age.Nvim Documentation])[
  #set page(paper: "a4", margin: 2.5cm)
  #set text(size: 11pt)

  #include "doc.typ"
  #include "license.typ"
] <pdf>


#font.export("FiraCode-Regular")