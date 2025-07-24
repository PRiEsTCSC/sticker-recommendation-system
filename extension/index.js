const API_BASE_URL = "http://localhost:8080"; // Replace with production URL in deployment

document.addEventListener("DOMContentLoaded", () => {
  const loginForm = document.getElementById("login-form");
  const signupForm = document.getElementById("signup-form");
  const loginMsg = document.getElementById("login-msg");
  const logoutBtn = document.getElementById("logout-btn");

  function updateUI(token, username) {
    if (token) {
      if (loginForm) loginForm.style.display = "none";
      if (signupForm) signupForm.style.display = "none";
      if (loginMsg) loginMsg.innerText = `✅ Logged in as ${username || "User"}`;
      if (logoutBtn) logoutBtn.style.display = "block";
    } else {
      if (loginForm) loginForm.style.display = "block";
      if (signupForm) signupForm.style.display = "block";
      if (loginMsg) loginMsg.innerText = "";
      if (logoutBtn) logoutBtn.style.display = "none";
    }
  }

  chrome.storage.local.get(["user_token", "username"], (result) => {
    updateUI(result.user_token, result.username);
  });

  if (loginForm) {
    loginForm.addEventListener("submit", async (e) => {
      e.preventDefault();
      const username = document.getElementById("login-username").value;
      const password = document.getElementById("login-password").value;
      if (loginMsg) loginMsg.innerText = "Logging in...";

      try {
        const res = await fetch(`${API_BASE_URL}/v1/auth/login/user`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ username, password }),
        });
        const data = await res.json();
        if (res.ok) {
          chrome.storage.local.set({ user_token: data.token, username: data.username }, () => {
            updateUI(data.token, data.username);
            chrome.runtime.sendMessage({ action: "updateMenu" });
          });
        } else {
          if (loginMsg) loginMsg.innerText = `❌ ${data.error || "Invalid credentials"}`;
        }
      } catch (err) {
        if (loginMsg) loginMsg.innerText = `❌ Network error: ${err.message}`;
      }
    });
  }

  if (signupForm) {
    signupForm.addEventListener("submit", async (e) => {
      e.preventDefault();
      const username = document.getElementById("signup-username").value;
      const password = document.getElementById("signup-password").value;
      if (loginMsg) loginMsg.innerText = "Signing up...";

      try {
        const res = await fetch(`${API_BASE_URL}/v1/auth/register/user`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ username, password }),
        });
        const data = await res.json();
        if (res.ok) {
          chrome.storage.local.set({ user_token: data.token, username: data.username }, () => {
            updateUI(data.token, data.username);
            chrome.runtime.sendMessage({ action: "updateMenu" });
          });
        } else {
          if (loginMsg) loginMsg.innerText = `❌ ${data.error || "Signup failed"}`;
        }
      } catch (err) {
        if (loginMsg) loginMsg.innerText = `❌ Network error: ${err.message}`;
      }
    });
  }

  if (logoutBtn) {
    logoutBtn.addEventListener("click", () => {
      chrome.storage.local.remove(["user_token", "username"], () => {
        updateUI(null, null);
        chrome.runtime.sendMessage({ action: "updateMenu" });
      });
    });
  }
});