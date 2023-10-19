# Asteroids

Asteroids is a game where you control a spaceship and shoot asteroids. The game is written in Rust (compiled to WebAssembly), Typescript and Python in the backend.  
The project is part of the modul "Webtechnologien" at the Technische Hochschule Lübeck.  

## How to run locally

### Backend

The backend is written in Python and uses the [Flask](https://flask.palletsprojects.com/en/2.0.x/) framework.

Prequisites:
- Python 3.9

To run the backend, navigate to the `backend` folder and run the following commands:
```
docker compose up -d
pip install -r requirements.txt
python main.py
```

### Frontend

The frontend is written in Rust and Typescript. The Rust code is compiled to WebAssembly and the Typescript code is compiled to Javascript using Webpack.

Prequisites:
- Rust
- cargo
- Node.js
- npm

To run the frontend, navigate to the `frontend` folder and run the following commands:
```
npm install
npm run start
```
