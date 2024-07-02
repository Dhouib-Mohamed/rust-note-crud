# Rust Note CRUD Application

Welcome to the Rust Note CRUD application! This project is a simple yet effective CRUD application written in Rust. It utilizes a PostgreSQL database to store notes and offers a RESTful API for performing CRUD operations.

## Pre-requisites

Before you begin, ensure you have the following tools installed:

- **Rust (version 1.61.0 or later):**
    - Verify installation with: `rustc --version`
- **Docker:**
    - Verify installation with: `docker --version`
- **Docker Compose:**
    - Verify installation with: `docker-compose --version`

## Project Structure

Here's a quick overview of the project's structure:

- `src/main.rs`: The main file that starts the application.
- `src/utils`: Contains utility functions.
- `src/router/handlers`: Handles API endpoint requests.
- `src/router/model.rs`: Defines data models for the API.
- `src/router/_index.rs`: Manages API routes.
- `src/db`: Manages database connection setup.

## Environment Variables

Ensure the following environment variables are set correctly:

- `DATABASE_URL`: URL for the PostgreSQL database. Default: `postgres://postgres:password@localhost:5432/notes`
- `PORT`: Port number for the application. Default: `8000`

## API Endpoints

The application provides the following API endpoints:

- `GET /health`: Check server health.
- `GET /note`: Retrieve all notes.
- `POST /note`: Create a new note.

## Setup Instructions

Follow these steps to set up and run the application:

1. **Clone the Repository:**
    ```bash
    git clone https://github.com/Dhouib-Mohamed/rust-note-crud.git
    ```
2. **Setup PostgreSQL Database Using Docker Compose:**
    ```bash
    cd db
    docker compose up -d
    ```
3. **Run the Application:**
    - **Locally in Development Mode:**
      - set the necessary environment variables in .env file
      - run: 
      ```bash
        cargo run
        ```
    - **In Release Mode:**
        ```bash
         docker build . --build-arg DATABASE_URL=postgres://user:password@localhost:5432/notes --build-arg PORT=8000 -t rust-crud && docker run --rm -p 8000:8000 rust-crud
        ```
4. **Access the Application:**
    - Open your browser and navigate to `http://localhost:8000` (replace the port number if you have set a different one in your environment variables).

5. **Test the Endpoints:**
    - **Check Server Health:**
        ```bash
        curl http://localhost:8000/health
        ```
    - **Create a Note:**
        ```bash
        curl -X POST http://localhost:8000/note -H "Content-Type: application/json" -d '{"title": "Note 1", "content": "Content of Note 1"}'
        ```
    - **Get All Notes:**
        ```bash
        curl http://localhost:8000/note
        ```

## Used Technologies

This project leverages the following technologies:

- **Rust**: A systems programming language focused on safety and performance.
- **Axum**: A fast, efficient, and modular web framework for Rust.
- **Tokio**: An asynchronous runtime for the Rust programming language.
- **PostgreSQL**: A powerful, open-source object-relational database system.
- **Docker**: A platform for developing, shipping, and running applications in containers.
- **Docker Compose**: A tool for defining and running multi-container Docker applications.
- **SQLx**: An async, pure Rust SQL crate featuring compile-time checked queries.
- **Serde**: A framework for serializing and deserializing Rust data structures.
- **dotenv**: Loads environment variables from a `.env` file.

## Troubleshooting

If you encounter any issues:

- Ensure all pre-requisites are installed correctly.
- Double-check that environment variables are set as needed.
- Verify that Docker containers are running properly.
- Check application logs for any runtime errors.

Feel free to reach out for help if you run into any problems or have questions!