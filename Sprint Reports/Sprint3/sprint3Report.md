# Sprint x Report (4/5/26 - 5/2/26)

## What's New (User Facing)
 * Admin dashboard: creating an easy, in-app way to edit item lists and info, and many other functions.
 * Recipt printing
 * General UI improvements

## Work Summary (Developer Facing)
Provide a one paragraph synposis of what your team accomplished this sprint. Don't repeat the "What's New" list of features. Instead, help the instructor understand how you went about the work described there, any barriers you overcame, and any significant learnings for your team.

Over sprint 3, Calvin worked on an Admin dashboard, accessable to certain accounts given an admin role by a previous admin, which allows for in-app editing of the database, and a view to the backend. Owen worked on adding receipt functionality and UI improvements.

## Unfinished Work
If applicable, explain the work you did not finish in this sprint. For issues/user stories in the current sprint that have not been closed, (a) any progress toward completion of the issues has been clearly tracked (by checking the checkboxes of  acceptance criteria), (b) a comment has been added to the issue to explain why the issue could not be completed (e.g., "we ran out of time" or "we did not anticipate it would be so much work"), and (c) the issue is added to a subsequent sprint, so that it can be addressed later.

There was the idea to integrate card payment to the app using an API, however this was phased out due to time constraints, and a similar fate met the ideas of making the program run as a progressive web app, and adding a function that could tell you exactly which coin denominations added up to the ammount of change required.

## Completed Issues/User Stories
Here are links to the issues that we completed in this sprint:

 * [Format Reciepts on Checkout](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/issues/38)
 * [Admin dashboard](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/issues/22)
 * [Keep list of inventory](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/issues/21)
 * [Add a feature to add stock](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/issues/19)
 
 ## Incomplete Issues/User Stories
 Here are links to issues we worked on but did not complete in this sprint:
 
* [Add change calculation on checkout screen](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/issues/39) Ran out of time, but also deprioritized as the sprint went on.
* [Make this into a PWA](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/issues/37) Similar issue as change calculation, deprioritized and ran out of time. 
* [Better payment methods](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/issues/24) This one was very close to being implemented, a good API was chosen, however it was too late to be added.

## Code Files for Review
Please review the following code files, which were actively developed during this sprint, for quality:
 * [admin/+page.svelte](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/blob/main/frontend/src/routes/admin/%2Bpage.svelte) (The main source for the admin view web page.)
 * [cashier/+page.svelte](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/blob/main/frontend/src/routes/cashier/%2Bpage.svelte) (Similar story for this file)
 * [models/product.rs](https://github.com/Tallented-Code-bot/CPTS322-Point-of-Sale-System/blame/main/backend/src/models/product.rs)
 
## Retrospective Summary
Here's what went well:
  * At the end of the semester, we're left with a well functioning app, with plenty of room for improvement, but isn't really broken. It just needs more features.
  * I believe the concept of what we were going for was started and was beginning to take pretty good shape.
  * We all learned a lot about different technologies and got better at working with others.
 
Here's what we'd like to improve:
   * Sprint 2 was definatly our most productive sprint, in terms of freqency of issues being completed. We slowed down during sprint 3 a bit.
