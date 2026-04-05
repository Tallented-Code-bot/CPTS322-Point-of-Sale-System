# Sprint x Report (3/2/26 - 4/4/26)

## What's New (User Facing)
 * Prettier user interface
 * Segmentation in interface allowing for a greater theoretical level of control.
 * Implementation of backend allowing for the program to be deployable as distinct client/server.

## Work Summary (Developer Facing)
<!-- Provide a one paragraph synposis of what your team accomplished this sprint. Don't repeat the "What's New" list of features. Instead, help the instructor understand how you went about the work described there, any barriers you overcame, and any significant learnings for your team. -->
Owen and Gio worked on improving the interface of the application. Calvin added a transaction API, allowing new items to be added to the backend, and set-up a hosting server for the app, found at [calvintallent.site](calvintallent.site). In this process, Calvin Tallent learned how to deploy a website on a Virtual Private Server (VPS) using Digital Ocean. Owen and Gio continued to learn much about CSS, HTML, and Svelte framework.

## Unfinished Work
<!-- If applicable, explain the work you did not finish in this sprint. For issues/user stories in the current sprint that have not been closed, (a) any progress toward completion of the issues has been clearly tracked (by checking the checkboxes of  acceptance criteria), (b) a comment has been added to the issue to explain why the issue could not be completed (e.g., "we ran out of time" or "we did not anticipate it would be so much work"), and (c) the issue is added to a subsequent sprint, so that it can be addressed later. -->
A feature called "Admin Dashboard" was planned to be done by sprint 2, however, the scope for this is much bigger than we originally thought, as it would be a near complete remake of the app. A feature called "Keep Inventory" was also planned, however, this was deemed unessecary as a sprint 2 feature in comparison to what was accomplished, seen as this is not a feature that most in-production POS systems are capable of. The last 3 uncompleted issues relate to maintaining a list of transactions in the backend which is part way done, but ultimatly had to be de-prioratized due to time.

## Completed Issues/User Stories
Here are links to the issues that we completed in this sprint:

 * [Clean-up UI on Checkout Screen](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/issues/28)
 * [Move 2nd screen functionality to 1st screen](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/issues/26)
 * [Unified UI design](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/issues/25)
 * [Second checkout screen](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/issues/23)
 * [Create Endpoint to Store Transactions](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/issues/8)
 * [Store transactions](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/issues/20)

Showcase videos in Sprint Demos.md

 <!-- Reminders (Remove this section when you save the file):
  * Each issue should be assigned to a milestone
  * Each completed issue should be assigned to a pull request
  * Each completed pull request should include a link to a "Before and After" video
  * All team members who contributed to the issue should be assigned to it on GitHub
  * Each issue should be assigned story points using a label
  * Story points contribution of each team member should be indicated in a comment -->
 
 ## Incomplete Issues/User Stories
 Here are links to issues we worked on but did not complete in this sprint:
 
 * [Admin dashboard](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/issues/22) Too big of scope for this stage of the project
 * [Keep list of inventory](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/issues/21) Deemed too big of scope, as well as unnessecary for our main goals.
 * [Build Transactions List View](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/issues/13)
 
 <!-- Examples of explanations (Remove this section when you save the file):
  * "We ran into a complication we did not anticipate (explain briefly)." 
  * "We decided that the feature did not add sufficient value for us to work on it in this sprint (explain briefly)."
  * "We could not reproduce the bug" (explain briefly).
  * "We did not get to this issue because..." (explain briefly) -->

## Code Files for Review
Please review the following code files, which were actively developed during this sprint, for quality:
 * [cart.ts](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/blob/main/frontend/src/lib/stores/cart.ts)
 * [transaction_api.rs](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/blob/main/backend/src/api/transaction_api.rs)
 * [main.rs](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/blob/main/backend/src/main.rs)
 
## Retrospective Summary
Here's what went well:
  * Time management
  * Project organization
  * Group communication
 
Here's what we'd like to improve:
   * Documenting issues
   * Keeping up on status of issues
  
Here are changes we plan to implement in the next sprint:
   * Printable formated output (recipts)
   * Cash and Card paths for payment (not nessicarilly the peripherals, but the states for each)
   * Improve security of transaction storage
