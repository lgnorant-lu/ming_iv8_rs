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

    function _portThis(self) {
        if (self == null || typeof self !== 'object' || !('_otherPort' in self)) {
            throw new TypeError('Illegal invocation');
        }
        return self;
    }

    MessagePort.prototype.postMessage = function postMessage(data) {
        var self = _portThis(this);
        var other = self._otherPort;
        if (other && other.onmessage) {
            var event = {data: data, origin: '', lastEventId: '', source: null, ports: []};
            // Use setTimeout to make it async (like real MessageChannel)
            var handler = other.onmessage;
            setTimeout(function() { handler(event); }, 0);
        }
    };

    MessagePort.prototype.start = function start() {
        _portThis(this)._started = true;
    };

    MessagePort.prototype.close = function close() {
        var self = _portThis(this);
        self.onmessage = null;
        self._otherPort = null;
    };

    MessagePort.prototype.addEventListener = function addEventListener(type, listener) {
        var self = _portThis(this);
        if (type === 'message') {
            self.onmessage = listener;
            self.start();
        }
    };

    MessagePort.prototype.removeEventListener = function removeEventListener() {
        _portThis(this);
    };

    function MessageChannel() {
        if (!(this instanceof MessageChannel)) {
            throw new TypeError("Failed to construct 'MessageChannel': Please use the 'new' operator");
        }
        this.port1 = new MessagePort();
        this.port2 = new MessagePort();
        this.port1._otherPort = this.port2;
        this.port2._otherPort = this.port1;
    }

    globalThis.MessageChannel = MessageChannel;
    globalThis.MessagePort = MessagePort;
})();
"#;
