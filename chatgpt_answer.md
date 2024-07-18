# ChatGPT based CRUD App

The following was produced from this prompt: `Generate list of endpoints, with JSON inputs and outputs, for a crud Todo Restful API`

> Certainly! Here's a list of CRUD endpoints for a TODO RESTful API with corresponding JSON inputs and outputs:
>
> ## Endpoints
>
> 1. **Create a TODO**
>    - **Endpoint:** `POST /todos`
>    - **Input JSON:**
>
>      ```json
>      {
>        "title": "String",
>        "description": "String",
>        "completed": "Boolean"
>      }
>      ```
>
>    - **Output JSON (created TODO):**
>
>      ```json
>      {
>        "id": "String",
>        "title": "String",
>        "description": "String",
>        "completed": "Boolean"
>      }
>      ```
>
> 2. **List all TODOs**
>    - **Endpoint:** `GET /todos`
>    - **Output JSON (array of TODOs):**
>
>      ```json
>      [
>        {
>          "id": "String",
>          "title": "String",
>          "description": "String",
>          "completed": "Boolean"
>        },
>        {
>          "id": "String",
>          "title": "String",
>          "description": "String",
>          "completed": "Boolean"
>        },
>        ...
>      ]
>      ```
>
> 3. **Get a TODO by ID**
>    - **Endpoint:** `GET /todos/{id}`
>    - **Output JSON (single TODO):**
>
>      ```json
>      {
>        "id": "String",
>        "title": "String",
>        "description": "String",
>        "completed": "Boolean"
>      }
>      ```
>
> 4. **Update a TODO**
>    - **Endpoint:** `PUT /todos/{id}`
>    - **Input JSON:**
>
>      ```json
>      {
>        "title": "String",
>        "description": "String",
>        "completed": "Boolean"
>      }
>      ```
>
>    - **Output JSON (updated TODO):**
>
>      ```json
>      {
>        "id": "String",
>        "title": "String",
>        "description": "String",
>        "completed": "Boolean"
>      }
>      ```
>
> 5. **Delete a TODO**
>    - **Endpoint:** `DELETE /todos/{id}`
>    - **Output JSON (deleted TODO):**
>
>      ```json
>      {
>        "id": "String",
>        "title": "String",
>        "description": "String",
>        "completed": "Boolean"
>      }
>      ```
>
> ## Notes
>
> - Replace `"String"` with actual string values, and `"Boolean"` with `true` or `false` as appropriate.
> - Use HTTP status codes (e.g., 200 for success, 201 for created, 404 for not found, etc.) appropriately.
> - Consider implementing pagination for the `GET /todos` endpoint if the number of TODOs is large.
> - Implement proper error handling and validation for requests and responses.
> - Depending on your application requirements, you may want to add additional endpoints or parameters (e.g., filtering, sorting) to enhance functionality.
