// Memora Companion - Background Service Worker
// Handles native messaging + smart suggestions

chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
    if (message.type === 'index_conversation') {
        chrome.runtime.sendNativeMessage(
            'com.memora.app',
            message.data,
            (response) => {
                if (chrome.runtime.lastError) {
                    console.error('Native messaging error:', chrome.runtime.lastError);
                }
            }
        );
    }
    
    if (message.type === 'get_smart_suggestions') {
        chrome.runtime.sendNativeMessage(
            'com.memora.app',
            { type: 'get_smart_suggestions', current_text: message.data.current_text },
            (response) => {
                if (!chrome.runtime.lastError && response) {
                    // Forward suggestions back to content script
                    chrome.tabs.sendMessage(sender.tab.id, {
                        type: 'smart_suggestions',
                        suggestions: response
                    });
                }
            }
        );
    }
    
    return true;
});

console.log('Memora background ready with smart suggestions');