This project aims to build a Crypto Mining Pool Restful API.

What is Restful API (Application Programming Interface)?

    - Enables two computer to exchange information ("Talk to each others")
    - There are different types of requests (CRUD):
        - POST: CREATE DATA     --> C
        - GET: READ DATA        --> R
        - PATCH: UPDATE DATA    --> U
        - DELETE: DELETE DATA   --> D
        - etc.
    - RESTful API:
        - Follow a set of rules/constraints known as Representational State Transfer
        - Organize resources/data into a bunch of unique URLs/URIs (Uniform Resource Identifiers) that differentiate differente type of resources on a server.
        - CLIENT -->REQUEST--> HOST/SERVER
            - REQUEST has a specific format:
                - HTTP VERB/URL_TO_ACCESS/ HEADERS(METADATA)/ BODY(Custom Pyaload of data)
        - HOST -->REQUEST--> CLIENT
            -  REQUEST format:
                - STATUS REQUEST/SERVER INFO/BODY(DATA PAYLOAD usually in JSON format)
        - STATELESS:
            - Each party doesn't need to store info about the others
            - Each request is independant from other communication
            - Predictable and reliable
