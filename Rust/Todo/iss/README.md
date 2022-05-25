# iss

## TODO

```text
pub enum Status {
        Open,
        Close,
    }


pub S {
        Author,
        Label: Option<Vec<LabelMeta>>, // Unlabeled, if None

        Assignee,
        Sort,
    }

    pub enum Sort {
Newest,
Oldest,
MostCommented,
LeastCommented,
RecentlyUpdated,
LeastRecentlyRpdated,
        }


pub Label {
        labelmeta:Option<Vec<LabelMeta>>,
    }
pub LabelMeta {
    name:String,
    tips:String,

    }

    impl Label {
           pub fn new() -> Self {
               let v = vec![
LabelMeta {

name: "bug".to_string(),
tios: Something isn't working
    }

documentation
Improvements or additions to documentation
duplicate
This issue or pull request already exists
enhancement
New feature or request
good first issue
Good for newcomers
help wanted
Extra attention is needed
help-wanted
invalid
This doesn't seem right
question
Further information is requested
wontfix
This will not be worked on

               ];

               }
        }
```
