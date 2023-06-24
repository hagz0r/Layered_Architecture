Simple todo project following layered architecture written in Rust

# How to run
install Rust

```cargo r```


# How to use
There's no interface except web-server at localhost.
So we need to run it from curl or smth

# Examples:

* Create a new task:

```curl -X POST -H "Content-Type: application/json" -d '{"title": "New Task", "description": "A new task to complete", "completed": false}' http://localhost:8080/tasks```

* Read all tasks:

```curl http://localhost:8080/tasks```

* Update a task:

```curl -X PUT -H "Content-Type: application/json" -d '{"title": "Updated Task", "description": "An updated task to complete", "completed": true}' http://localhost:8080/tasks/1```

* Delete a task:

```curl -X DELETE http://localhost:8080/tasks/1```
