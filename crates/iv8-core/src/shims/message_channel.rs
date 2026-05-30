//! MessageChannel polyfill.
//!
//! Provides MessageChannel + MessagePort for postMessage communication.
//! Used by many anti-bot scripts (abogus, h5st) for async communication patterns.

/// JS shim for MessageChannel.
pub const MESSAGE_CHANNEL_JS: &str = r#"
(function() {
    function MessagePort() {
        this.onmessage = null;
        this.onmessageerror = null;
        this._otherPort = null;
        this._started = false;
    }

    MessagePort.prototype.postMessage = function(data) {
        var other = this._otherPort;
        if (other && other.onmessage) {
            var event = {data: data, origin: '', lastEventId: '', source: null, ports: []};
            // Use setTimeout to make it async (like real MessageChannel)
            var handler = other.onmessage;
            setTimeout(function() { handler(event); }, 0);
        }
    };

    MessagePort.prototype.start = function() {
        this._started = true;
    };

    MessagePort.prototype.close = function() {
        this.onmessage = null;
        this._otherPort = null;
    };

    MessagePort.prototype.addEventListener = function(type, listener) {
        if (type === 'message') {
            this.onmessage = listener;
            this.start();
        }
    };

    MessagePort.prototype.removeEventListener = function() {};

    function MessageChannel() {
        this.port1 = new MessagePort();
        this.port2 = new MessagePort();
        this.port1._otherPort = this.port2;
        this.port2._otherPort = this.port1;
    }

    globalThis.MessageChannel = MessageChannel;
    globalThis.MessagePort = MessagePort;
})();
"#;
