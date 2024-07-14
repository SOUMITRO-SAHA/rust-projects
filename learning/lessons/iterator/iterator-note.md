# Iterator

- Allows to perform task on **sequence** of items **in turn**
- Iterators are **lazy**, meaning **no effect** until methods are called that **consume** the iterator to use it up
- All iterators implement trait **Iterator** which has `next()` method, which gets called automatically when traversing over some data
- Some methods **consume** iterators while **produce a new iterator** from the provided iterator
