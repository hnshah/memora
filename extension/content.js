// Memora Companion - Content Script
// Smart live context injection (polished UI)

console.log('Memora Companion loaded with polished smart suggestions');

let debounceTimer = null;
let currentSuggestions = [];
const SITE = window.location.hostname;

// ... (existing helper functions remain)

function showSmartSuggestions(suggestions) {
    const old = document.getElementById('memora-smart-suggestions');
    if (old) old.remove();

    if (!suggestions || suggestions.length === 0) return;
    currentSuggestions = suggestions;

    const container = document.createElement('div');
    container.id = 'memora-smart-suggestions';
    container.style.cssText = `
        position: fixed;
        bottom: 90px;
        right: 24px;
        background: #0f0f0f;
        border: 1px solid #2a2a2a;
        border-radius: 14px;
        padding: 8px;
        max-width: 340px;
        z-index: 999999;
        box-shadow: 0 20px 60px rgba(0,0,0,0.5);
        font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
        animation: memoraFadeIn 0.15s ease-out;
    `;

    let html = `<div style="padding: 6px 12px; font-size: 11px; color: #666; letter-spacing: 0.5px;">Smart context from your memory</div>`;

    suggestions.forEach((s, index) => {
        html += `
            <div class="memora-suggestion" data-index="${index}"
                 style="padding: 11px 14px; margin: 4px 0; background: #1a1a1a; border-radius: 10px; cursor: pointer; transition: all 0.1s ease; border: 1px solid transparent;">
                <div style="font-size: 13.5px; line-height: 1.35; color: #e5e5e5; margin-bottom: 4px;">
                    ${s.content.substring(0, 130)}${s.content.length > 130 ? '...' : ''}
                </div>
                <div style="font-size: 10.5px; color: #777; display: flex; align-items: center; gap: 6px;">
                    <span style="background: #2a2a2a; padding: 1px 6px; border-radius: 4px; font-size: 9.5px;">${s.source}</span>
                    <span>${s.relevance_reason}</span>
                </div>
            </div>
        `;
    });

    container.innerHTML = html;
    document.body.appendChild(container);

    // Hover effects
    container.querySelectorAll('.memora-suggestion').forEach(el => {
        el.addEventListener('mouseenter', () => {
            el.style.background = '#242424';
            el.style.borderColor = '#3a3a3a';
        });
        el.addEventListener('mouseleave', () => {
            el.style.background = '#1a1a1a';
            el.style.borderColor = 'transparent';
        });
        el.addEventListener('click', () => {
            const index = parseInt(el.dataset.index);
            injectContext(currentSuggestions[index].content);
        });
    });

    // Click outside to close
    setTimeout(() => {
        document.addEventListener('click', function handler(e) {
            if (!container.contains(e.target)) {
                container.remove();
                document.removeEventListener('click', handler);
            }
        }, { once: true });
    }, 100);

    // Auto-hide after 12 seconds
    setTimeout(() => {
        if (container.parentNode) container.parentNode.removeChild(container);
    }, 12000);
}

function injectContext(content) {
    const input = document.querySelector('textarea, div[contenteditable="true"], [data-testid="chat-input"]');
    if (input) {
        const prefix = input.value !== undefined ? input.value : input.innerText;
        if (input.value !== undefined) {
            input.value = content + '\n\n' + prefix;
        } else {
            input.innerText = content + '\n\n' + prefix;
        }
        input.dispatchEvent(new Event('input', { bubbles: true }));
    }
    const suggestionsUI = document.getElementById('memora-smart-suggestions');
    if (suggestionsUI) suggestionsUI.remove();
}

// Real-time monitoring + smart suggestions
setInterval(() => {
    const text = getCurrentInputText();
    if (text.length > 12 && text !== window.lastSentText) {
        window.lastSentText = text;
        chrome.runtime.sendMessage({
            type: 'get_smart_suggestions',
            data: { current_text: text }
        });
    }
}, 1100);

chrome.runtime.onMessage.addListener((message) => {
    if (message.type === 'smart_suggestions') {
        showSmartSuggestions(message.suggestions);
    }
});