/* 
yew doesn't have examples for session/cookie control.
Since session is managed by "backend/serverside", cookie handling is going to be done in this section.

In this template, we use "reqwasm" for HTTP requests.
Example will be shown using the library's example.
 */
use reqwasm::http::Request;

// Sets the request credentials.