const gate = document.querySelector("#gate");
const addressForm = document.querySelector("#addressForm");
const addressInput = document.querySelector("#addressInput");
const gateError = document.querySelector("#gateError");
const currentAddress = document.querySelector("#currentAddress");
const recipientInput = document.querySelector("#recipientInput");
const chatMain = document.querySelector("#chatMain");
const emptyState = document.querySelector("#emptyState");
const messages = document.querySelector("#messages");
const messageForm = document.querySelector("#messageForm");
const messageInput = document.querySelector("#messageInput");
const sendButton = document.querySelector("#sendButton");
const hint = document.querySelector("#hint");

const markdown = window.markdownit
    ? window.markdownit({
          breaks: true,
          html: false,
          linkify: true,
      })
    : null;

let socket = null;
let address = "";
let connecting = false;
let connected = false;

addressInput.focus();

addressForm.addEventListener("submit", (event) => {
    event.preventDefault();
    const nextAddress = addressInput.value.trim();
    if (connecting) {
        return;
    }
    if (!nextAddress) {
        gateError.textContent = "Введите адрес пользователя.";
        return;
    }
    connect(nextAddress);
});

messageForm.addEventListener("submit", (event) => {
    event.preventDefault();
    submitMessage();
});

messageInput.addEventListener("input", () => {
    resizeComposer();
    syncSendButton();
});

messageInput.addEventListener("keydown", (event) => {
    if (event.key !== "Enter" || event.shiftKey) {
        return;
    }
    event.preventDefault();
    submitMessage();
});

recipientInput.addEventListener("input", syncSendButton);

addressInput.addEventListener("input", () => {
    gateError.textContent = "";
});

document.addEventListener("click", (event) => {
    if (event.target.closest(".menu-anchor")) {
        return;
    }
    closeMenus();
});

function connect(nextAddress) {
    socket?.close();
    connecting = true;
    connected = false;
    addressInput.disabled = true;
    gateError.textContent = "";

    const url = getWebSocketUrl();
    if (!url) {
        gateError.textContent = "Откройте страницу через client server, не как локальный файл.";
        connecting = false;
        addressInput.disabled = false;
        return;
    }

    socket = new WebSocket(url);
    const currentSocket = socket;

    socket.addEventListener("open", () => {
        if (socket !== currentSocket) {
            return;
        }
        address = nextAddress;
        socket.send(nextAddress);
        currentAddress.textContent = nextAddress;
        gate.hidden = true;
        gate.style.display = "none";
        connecting = false;
        connected = true;
        addressInput.disabled = false;
        syncSendButton();
        messageInput.focus();
    });

    socket.addEventListener("message", (event) => {
        if (socket !== currentSocket) {
            return;
        }
        handleServerMessage(event.data);
    });

    socket.addEventListener("close", () => {
        if (socket !== currentSocket) {
            return;
        }
        const wasConnected = connected;
        connecting = false;
        connected = false;
        addressInput.disabled = false;
        if (wasConnected) {
            gate.hidden = false;
            gate.style.display = "";
            gateError.textContent = "WebSocket закрыт.";
        } else {
            gateError.textContent = "WebSocket не подключен.";
        }
        syncSendButton();
    });

    socket.addEventListener("error", () => {
        if (socket !== currentSocket || connected) {
            return;
        }
        connecting = false;
        addressInput.disabled = false;
    });
}

function submitMessage() {
    const body = messageInput.value.trim();
    const to = recipientInput.value.trim();
    if (!body) {
        syncSendButton();
        return;
    }
    if (!to) {
        appendSystemMessage("Укажите адрес получателя в верхней строке.");
        return;
    }
    if (!socket || socket.readyState !== WebSocket.OPEN) {
        appendSystemMessage("WebSocket не подключен.");
        return;
    }

    socket.send(JSON.stringify({ to, body }));
    appendMessage({
        role: "user",
        from: address,
        to,
        body,
        createdAt: new Date(),
    });

    messageInput.value = "";
    resizeComposer();
    syncSendButton();
}

function handleServerMessage(payload) {
    let message;
    try {
        message = JSON.parse(payload);
    } catch {
        showServerError("Сервер прислал некорректный JSON.");
        return;
    }

    if (message.type === "ready") {
        return;
    }

    if (message.type === "error" && typeof message.message === "string") {
        showServerError(message.message);
        return;
    }

    if (
        typeof message.from !== "string" ||
        typeof message.to !== "string" ||
        typeof message.body !== "string"
    ) {
        showServerError("Сервер прислал сообщение неизвестного формата.");
        return;
    }

    appendMessage({
        role: "peer",
        from: message.from,
        to: message.to,
        body: message.body,
        createdAt: new Date(),
    });
}

function showServerError(message) {
    if (connected) {
        appendSystemMessage(message);
        return;
    }
    gateError.textContent = message;
    connecting = false;
    addressInput.disabled = false;
}

function appendSystemMessage(body) {
    appendMessage({
        role: "system",
        body,
        createdAt: new Date(),
    });
}

function appendMessage(message) {
    emptyState.hidden = true;
    messages.append(createMessageElement(message));
    requestAnimationFrame(() => {
        chatMain.scrollTo({
            top: chatMain.scrollHeight,
            behavior: "smooth",
        });
    });
}

function createMessageElement(message) {
    const article = document.createElement("article");
    const meta = document.createElement("div");
    const bubble = document.createElement("div");

    article.className = `message ${message.role}`;
    meta.className = "message-meta";
    meta.textContent = message.from || "";

    bubble.className = "bubble";
    if (message.role === "peer") {
        bubble.classList.add("markdown-body");
        if (markdown && window.DOMPurify) {
            bubble.innerHTML = window.DOMPurify.sanitize(markdown.render(message.body));
        } else {
            bubble.textContent = message.body;
        }
    } else {
        bubble.textContent = message.body;
    }

    article.append(meta, bubble, createActions(message));
    return article;
}

function createActions(message) {
    const actions = document.createElement("div");
    const copyButton = document.createElement("button");
    const menuAnchor = document.createElement("div");
    const menuButton = document.createElement("button");
    const menu = createMenu(message);

    actions.className = "message-actions";

    copyButton.className = "icon-button";
    copyButton.type = "button";
    copyButton.ariaLabel = "Скопировать";
    copyButton.innerHTML = icon("copy");
    copyButton.addEventListener("click", () => {
        navigator.clipboard.writeText(message.body);
    });

    menuAnchor.className = "menu-anchor";
    menuButton.className = "icon-button";
    menuButton.type = "button";
    menuButton.ariaLabel = "Открыть меню";
    menuButton.innerHTML = icon("dots");
    menuButton.addEventListener("click", (event) => {
        event.stopPropagation();
        closeMenus(menu);
        menu.hidden = !menu.hidden;
    });

    menuAnchor.append(menuButton, menu);
    actions.append(copyButton, menuAnchor);
    return actions;
}

function createMenu(message) {
    const menu = document.createElement("div");
    const time = document.createElement("div");

    menu.className = "message-menu";
    menu.hidden = true;

    time.className = "menu-time";
    time.textContent = `сегодня, ${formatTime(message.createdAt)}`;

    menu.append(time);
    return menu;
}

function closeMenus(except = null) {
    document.querySelectorAll(".message-menu").forEach((menu) => {
        if (menu !== except) {
            menu.hidden = true;
        }
    });
}

function resizeComposer() {
    messageInput.style.height = "0px";
    messageInput.style.height = `${Math.min(messageInput.scrollHeight, 180)}px`;
}

function syncSendButton() {
    const hasRecipient = recipientInput.value.trim().length > 0;
    const hasBody = messageInput.value.trim().length > 0;
    sendButton.disabled = !connected || !hasBody;
    hint.classList.toggle("is-hidden", hasRecipient);
}

function getWebSocketUrl() {
    if (!window.location.host) {
        return null;
    }
    const protocol = window.location.protocol === "https:" ? "wss" : "ws";
    return `${protocol}://${window.location.host}/api/ws`;
}

function formatTime(date) {
    return date.toLocaleTimeString("ru-RU", {
        hour: "2-digit",
        minute: "2-digit",
    });
}

function icon(name) {
    const icons = {
        copy: `
            <svg viewBox="0 0 24 24" aria-hidden="true">
                <rect x="8" y="8" width="11" height="11" rx="2"></rect>
                <path d="M5 15V7a2 2 0 0 1 2-2h8"></path>
            </svg>
        `,
        dots: `
            <svg viewBox="0 0 24 24" aria-hidden="true">
                <circle cx="5" cy="12" r="1.8"></circle>
                <circle cx="12" cy="12" r="1.8"></circle>
                <circle cx="19" cy="12" r="1.8"></circle>
            </svg>
        `,
    };
    return icons[name];
}
