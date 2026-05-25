// Memora Companion - Content Script
// Smart live context injection (real-time suggestions)

console.log('Memora Companion loaded with smart suggestions');

let debounceTimer = null;
const SITE = window.location.hostname;

// ... (existing extract functions for different sites remain)

function getCurrentInputText() {
    // Try common input selectors for major AI sites
    const selectors = [
        'textarea', 
        'div[contenteditable="true"]', 
        'input[type="text"]', 
        '[data-testid="chat-input"]', 
        '.ProseMirror'
    ];
    
    for (const selector of selectors) {
        const el = document.querySelector(selector);
        if (el && el.value) return el.value.trim();
        if (el && el.innerText) return el.innerText.trim();
    }
    return '';
}

function showSmartSuggestions(suggestions) {
    // Remove old suggestion UI
    const old = document.getElementById('memora-smart-suggestions');
    if (old) old.remove();

    if (!suggestions || suggestions.length === 0) return;

    const container = document.createElement('div');
    container.id = 'memora-smart-suggestions';
    container.style.cssText = 'position:fixed;bottom:80px;right:20px;background:#111;border:1px solid #333;border-radius:12px;padding:12px;max-width:320px;z-index:99999;box-shadow:0 10px 30px rgba(0,0,0,0.4)';
    
    let html = '<div style="font-size:12px;color:#888;margin-bottom:8px">Smart suggestions from your memory</div>';
    
    suggestions.forEach(s => {
        html += `
            <div onclick="injectContext('${s.content.replace(/'/g, "\\'")}')" 
                 style="padding:10px;margin:6px 0;background:#1a1a1a;border-radius:8px;cursor:pointer;border:1px solid #333">
                <div style="font-size:13px;line-height:1.4;color:#ddd">${s.content.substring(0, 140)}${s.content.length > 140 ? '...' : ''}</div>
                <div style="font-size:10px;color:#666;margin-top:4px">${s.source} • ${s.relevance_reason}</div>
            </div>
        `;
    });
    
    container.innerHTML = html;
    document.body.appendChild(container);
    
    // Auto-hide after 8 seconds
    setTimeout(() => { if (container.parentNode) container.parentNode.removeChild(container); }, 8000);
}

function injectContext(content) {
    // Find the active input and insert the context
    const input = document.querySelector('textarea, div[contenteditable="true"], [data-testid="chat-input"]');
    if (input) {
        if (input.value !== undefined) {
            input.value = content + '\n\n' + input.value;
        } else {
            input.innerText = content + '\n\n' + input.innerText;
        }
        input.dispatchEvent(new Event('input', { bubbles: true }));
    }
    // Remove suggestion UI
    const suggestionsUI = document.getElementById('memora-smart-suggestions');
    if (suggestionsUI) suggestionsUI.remove();
}

// Real-time input monitoring
setInterval(() => {
    const text = getCurrentInputText();
    if (text.length > 15 && text !== window.lastSentText) {
        window.lastSentText = text;
        
        // Send to backend for smart suggestions
        chrome.runtime.sendMessage({
            type: 'get_smart_suggestions',
            data: { current_text: text }
        });
    }
}, 1200);

// Listen for smart suggestions from background
chrome.runtime.onMessage.addListener((message) => {
    if (message.type === 'smart_suggestions') {
        showSmartSuggestions(message.suggestions);
    }
});