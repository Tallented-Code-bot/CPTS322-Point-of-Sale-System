# Sprint x Report (8/26/21 - 9/24/2021)
## What's New (User Facing)
* Implemented the Cashier Checkout user interface page
* Implemented the Auth0 login library
* Implemented MongoDB API calls to retrieve items
## Work Summary (Developer Facing)
For sprint 1, Gio focused on building out the frontend structure for the Cashier Checkout route using SvelteKit. The main goal was to get the UI fully structured and functional before deeper backend integration.
Owen worked on linking the various components and routes within Svelte and managing login status throughout the program. Calvin implemented the Auth0 login library and the Rust API for the frontend to communicate to the MongoDB backend.
## Unfinished Work
* Additional UI polish and validation (such as improving empty cart behavior) will be addressed next sprint.
* The sectioning of data to only be added on the "home" page was not realized.
* While users are able to login, this info is not made use of as there are no features which require elevated permissions and roles yet. There is also no user whitelist, so you cannot fail to login at this time.
* Transactions are not stored at this time.
## Completed Issues/User Stories
Here are links to the issues that we completed in this sprint:
* Cart and summary panel layout development
* URL of issue 2
* URL of issue n
## Incomplete Issues/User Stories
Here are links to issues we worked on but did not complete in this sprint:
* URL of issue 1 <<One sentence explanation of why issue was not completed>>
* URL of issue 2 <<One sentence explanation of why issue was not completed>>
* URL of issue n <<One sentence explanation of why issue was not completed>>
Examples of explanations (Remove this section when you save the file):
* "We ran into a complication we did not anticipate (explain briefly)."
* "We decided that the feature did not add sufficient value for us to work on it
in this sprint (explain briefly)."
* "We could not reproduce the bug" (explain briefly).
* "We did not get to this issue because..." (explain briefly)
## Code Files for Review
Please review the following code files, which were actively developed during this
sprint, for quality:
* [Name of code file 1]([https://github.com/frontend/src/routes/cashier/+page.svelte](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/blob/main/frontend/src/routes/cashier/%2Bpage.svelte))
* [Name of code file 2]([https://github.com/your_repo/file_extension](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/blob/main/frontend/src/routes/%2Bpage.svelte))
* [Name of code file 3]([https://github.com/your_repo/file_extension](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/blob/main/frontend/src/routes/home/%2Bpage.svelte))
## Retrospective Summary
Here's what went well:
* Given the real time spent working, a very good amount of progress was made.
* We remain well-documented, with a solid idea of where we want the project to be throughout time.
* Our team got off the ground early with meetings, allowing us to get a little bit of an organizational head start.
##
Here's what we'd like to improve:
* Begin backend integration earlier in the sprint to avoid last minute debugging.
* More consistant development overall to avoid finishing sprints as literal sprints, developing more as a marathon.
* Better use of Github's project organization features such as issues and branches, as well as more team-oriented organization over discord.
##
Here are changes we plan to implement in the next sprint:
* A more polished UI (Customized Login screen, cohesive design decisions throuout different elements of the UI).
* More seperated interface states, specifically the 'home' page, with the intended purpose of being where items are added to a transaction, and the 'cashier' page where the costs and fees are calcuated.
