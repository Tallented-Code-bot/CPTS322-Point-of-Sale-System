# CPTS322-Point-of-Sale-System

things needed:
- backend
  - database storing product information
    - price
    - quantity
    - upc
  - 

- frontend
  - cashier page



stack:
- svelte for frontend (combination of html and javascript)
- mongoDB for database
- rust for backend with rocket library



## Developing

You will need node.js to build the frontend, and rust to build the backend. 
- [Install rust](https://rust-lang.org/tools/install/)
- [Install node](https://nodejs.org/en/download) (or I recommend `nvm` (node version manager) instead)

To get started, clone the repo, then install dependencies.
```sh
git clone https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System

# For the frontend
cd CPTS322-Point-of-Sale-System/frontend
npm install
npm run build

# For the backend
cd ../backend/
cargo build
cargo run # the application should now work.
```

Now open <http://127.0.0.1:8000> in your browser to view the app.


### Report
To view the report, install the [tectonic
compiler](https://tectonic-typesetting.github.io/book/latest/installation/),
then run `tectonic -X build` in the `report` directory.
