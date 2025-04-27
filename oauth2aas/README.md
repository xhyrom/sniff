# OAuth2AAS Token Generator

A simple tool to obtain Android Authentication Service (AAS) tokens using Google Play API.

## Usage

1. **Get an OAuth2 token:**

   - Open [Google's embedded setup page](https://accounts.google.com/EmbeddedSetup) in your browser
   - Log in with your Google account
   - Access browser developer tools (F12 or right-click -> Inspect)
   - Go to Local Storage tab and copy the oauth2 token

2. **Run the tool:**

   ```bash
   cargo run --package oauth2aas <your-email@gmail.com> <oauth2-token>
   ```
