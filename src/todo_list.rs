use crate::*;
// use std::borrow::Borrow;
use std::fmt;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Index(u64);

impl Index {
    pub fn new(i: u64) -> Index {
        Index(i)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Description(String);

impl Description {
    pub fn new(s: &str) -> Description {
        Description(s.to_owned())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Description {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag(String);

impl Tag {
    pub fn new(s: &str) -> Tag {
        Tag(s.to_owned())
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn from_strings(ss: Vec<&str>) -> Vec<Tag> {
        ss.clone().into_iter().map(|s| Tag::new(s)).collect()
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoItem {
    pub index: Index,
    pub description: Description,
    pub tags: Vec<Tag>,
    pub done: bool,
}

impl TodoItem {
    pub fn new(index: Index, description: Description, tags: Vec<Tag>, done: bool) -> TodoItem {
        TodoItem {
            index,
            description,
            tags,
            done,
        }
    }
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}, {:?}", self.index, self.description, self.tags)
    }
}

#[derive(Debug, Clone)]
pub struct TodoList {
    top_index: Index,
    items: Arc<Mutex<Vec<TodoItem>>>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            top_index: Index::new(0),
            items: Arc::new(Mutex::new(vec![])),
        }
    }

    pub async fn push(&mut self, description: Description, tags: Vec<Tag>) -> TodoItem {
        // unimplemented!();
        let index = self.top_index;
        self.top_index = Index::new(index.value() + 1);

        let new_item = TodoItem::new(index, description, tags, false);
        self.items.lock().await.push(new_item.clone());
        new_item
    }

    pub async fn done_with_index(&mut self, idx: Index) -> Option<Index> {
        // unimplemented!();
        if let Some(item) = self
            .items
            .lock()
            .await
            .iter_mut()
            .find(|item| item.index == idx)
        {
            item.done = true;
            Some(idx)
        } else {
            None
        }
    }

    pub async fn search(&self, sp: SearchParams) -> Vec<TodoItem> {
        let mut result: Vec<TodoItem> = Vec::new();
        for item in self.items.lock().await.iter_mut() {
            // Check if item description contains all search words
            let description_matches = sp.words.iter().all(|search_word| {
                let desc_words = item.description.value().split(" ");
                let mut res = false;
                for word in desc_words {
                    let mut desc_iter = word.chars().peekable();

                    // Iterate through the characters of the search word
                    let mut l = 0;
                    for search_char in search_word.0.chars() {
                        // Check if the current character in the search word matches the current character in the description
                        while let Some(desc_char) = desc_iter.next() {
                            if desc_char == search_char {
                                l = l + 1;
                                break;
                            }
                        }
                    }

                    // If all characters of the search word were found in order, and the item is not marked as done, return true
                    if l == search_word.0.len() && !item.done {
                        res = true;
                        break;
                    }
                }
                res
            });

            let tags_match = sp.tags.iter().all(|tag| {
                let mut res = false;
                for i_tag in item.tags.clone() {
                    let mut desc_iter = i_tag.value().chars().peekable();
                    let mut l = 0;
                    for search_char in tag.0.chars() {
                        while let Some(desc_char) = desc_iter.next() {
                            if desc_char == search_char {
                                l = l + 1;
                                break;
                            }
                        }
                    }
                    if l == tag.0.len() && !item.done {
                        res = true;
                        break;
                    }
                }
                res
            });

            if sp.words.len() != 0 && sp.tags.len() != 0 {
                if description_matches && tags_match {
                    result.push(item.clone());
                }
            } else if (description_matches && sp.words.len() != 0)
                || (tags_match && sp.tags.len() != 0)
            {
                result.push(item.clone());
            }
        }

        result
    }
}
