const API_BASE_URL = "http://127.0.0.1:8080"; // Replace with production URL in deployment

chrome.runtime.onInstalled.addListener((details) => {
  if (details.reason === "install" || details.reason === "update") {
    createContextMenuIfLoggedIn();
  }
});

function createContextMenu() {
  chrome.contextMenus.create({
    id: "find-sticker",
    title: "Find Sticker",
    contexts: ["selection"],
    documentUrlPatterns: ["https://x.com/*"]
  });
}

function createContextMenuIfLoggedIn() {
  chrome.storage.local.get(["user_token"], (result) => {
    if (result.user_token) {
      createContextMenu();
    }
  });
}

chrome.contextMenus.onClicked.addListener((info, tab) => {
  if (info.menuItemId === "find-sticker" && info.selectionText) {
    chrome.storage.local.get(["user_token", "username"], (result) => {
      const { user_token: token, username } = result;

      if (!token || !username) {
        chrome.notifications.create({
          type: "basic",
          iconUrl: "icon.png",
          title: "Sticker Finder",
          message: "Please log in via the extension popup."
        });
        return;
      }

      fetch(`${API_BASE_URL}/v1/sticker/find`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "Authorization": `Bearer ${token}`
        },
        body: JSON.stringify({ input_text: info.selectionText, username })
      })
        .then((res) => {
          if (!res.ok) {
            throw new Error(`HTTP error! status: ${res.status}`);
          }
          return res.json();
        })
        .then((data) => {
          if (data.sticker_urls && data.sticker_urls.length > 0) {
            // Open a new tab for each sticker URL
            data.sticker_urls.forEach((url) => {
              if (url) {
                chrome.tabs.create({ url });
              }
            });
          } else {
            // Notify the user if no stickers are found
            chrome.notifications.create({
              type: "basic",
              iconUrl: "icon.png",
              title: "Sticker Finder",
              message: "No stickers found."
            });
          }
        })
        .catch((err) => {
          // Handle errors with a notification
          chrome.notifications.create({
            type: "basic",
            iconUrl: "icon.png",
            title: "Sticker Finder",
            message: `Error: ${err.message}`
          });
        });
    });
  }
});

chrome.storage.onChanged.addListener((changes, namespace) => {
  if (namespace === "local" && changes.user_token) {
    chrome.contextMenus.removeAll(() => {
      if (changes.user_token.newValue) {
        createContextMenu();
      }
    });
  }
});