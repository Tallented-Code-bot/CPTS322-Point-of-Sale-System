# CPTS322-Point-of-Sale-System

A web-based point of sale system.

---

Are you tired of slow, proprietary, and outdated point of sale software? So were we! Cascade Secure POS aims to mitigate this problem by creating a lightweight, secure POS system with a client server approach, meaning it can run on anything with an up-to-date and reasonably featured internet browser. 

<!-- things needed:
- backend
  - database storing product information
    - price
    - quantity
    - upc
  - 

- frontend
  - cashier page

-->

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


### Functionality
Currently, the website is deployable and functional in of itself, although it is not very polished, and lacks any meaninful support for expected peripherals such as a recipt printer.

Once you set up a backend with the provided instructions, you can add items to your cart and check them out.

### Known Problems

### Licence
MIT License

Copyright (c) 2026 Owen Moore, Calvin Tallent, Giovanni Munoz

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
