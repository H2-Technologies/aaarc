document.addEventListener("DOMContentLoaded", function () {
  const descopeSdk = Descope({ projectId: "__ProjectID__" });

  const sessionToken = descopeSdk.getSessionToken();

  // example fetch call with authentication header
  fetch("127.0.0.1:8000/auth/clientSessionToken", {
    headers: {
      Accept: "application/json",
      Authorization: "Bearer " + sessionToken,
    },
  });
});
