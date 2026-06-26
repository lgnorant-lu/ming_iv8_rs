//! AudioContext / OfflineAudioContext / Web Audio API stubs.
//!
//! FingerprintJS uses AudioContext for audio fingerprinting.
//! Extracted from document_props.rs for code organization.
//!
//! Dependencies: performance.now(), setTimeout (events/timers.rs)

pub const AUDIO_CONTEXT_JS: &str = r#"
(function() {
    // AudioParam stub
    function AudioParam(value) {
        throw new TypeError('Illegal constructor');
        _initAudioParam(this, value);
    }
    function _initAudioParam(self, value) {
        self.value = value !== undefined ? value : 0;
        self.defaultValue = self.value;
        self.minValue = -3.4028234663852886e+38;
        self.maxValue = 3.4028234663852886e+38;
        self.automationRate = 'a-rate';
    }
    AudioParam.prototype.setValueAtTime = function(v, t) { this.value = v; return this; };
    AudioParam.prototype.linearRampToValueAtTime = function(v, t) { return this; };
    AudioParam.prototype.exponentialRampToValueAtTime = function(v, t) { return this; };
    AudioParam.prototype.setTargetAtTime = function(v, t, tc) { return this; };
    AudioParam.prototype.setValueCurveAtTime = function(vs, t, d) { return this; };
    AudioParam.prototype.cancelScheduledValues = function(t) { return this; };
    AudioParam.prototype.cancelAndHoldAtTime = function(t) { return this; };

    // Internal factory: creates AudioParam without requiring `new` (bypasses
    // the illegal-constructor guard). Used by node constructors and factory
    // methods so external `new AudioParam()` throws but internal creation works.
    function _createAudioParam(value) {
        var p = Object.create(AudioParam.prototype);
        _initAudioParam(p, value);
        return p;
    }

    // AudioNode base
    function AudioNode(ctx) {
        throw new TypeError('Illegal constructor');
        _initAudioNode(this, ctx);
    }
    function _initAudioNode(self, ctx) {
        self.context = ctx;
        self.numberOfInputs = 0;
        self.numberOfOutputs = 1;
        self.channelCount = 2;
        self.channelCountMode = 'max';
        self.channelInterpretation = 'speakers';
    }
    function _createAudioNode(ctx) {
        var node = Object.create(AudioNode.prototype);
        _initAudioNode(node, ctx);
        return node;
    }
    AudioNode.prototype.connect = function(dest) { return dest; };
    AudioNode.prototype.disconnect = function() {};
    AudioNode.prototype.addEventListener = function() {};
    AudioNode.prototype.removeEventListener = function() {};
    AudioNode.prototype.dispatchEvent = function() { return true; };

    // OscillatorNode
    function OscillatorNode(ctx, options) {
        throw new TypeError('Illegal constructor');
        _initOscillatorNode(this, ctx, options);
    }
    function _initOscillatorNode(self, ctx, options) {
        AudioNode.call(self, ctx);
        self.type = (options && options.type) || 'sine';
        self.frequency = _createAudioParam((options && options.frequency) || 440);
        self.detune = _createAudioParam(0);
        self.onended = null;
    }
    function _createOscillatorNode(ctx, options) {
        var node = Object.create(OscillatorNode.prototype);
        _initOscillatorNode(node, ctx, options);
        return node;
    }
    OscillatorNode.prototype = Object.create(AudioNode.prototype);
    OscillatorNode.prototype.start = function(when) {};
    OscillatorNode.prototype.stop = function(when) {};

    // DynamicsCompressorNode
    function DynamicsCompressorNode(ctx, options) {
        throw new TypeError('Illegal constructor');
        _initDynamicsCompressorNode(this, ctx, options);
    }
    function _initDynamicsCompressorNode(self, ctx, options) {
        AudioNode.call(self, ctx);
        var _comp = (_audioPrefs && _audioPrefs.compressor) ? _audioPrefs.compressor : {};
        self.threshold = _createAudioParam((options && options.threshold !== undefined) ? options.threshold : (_comp.threshold !== undefined ? _comp.threshold : -24));
        self.knee = _createAudioParam((options && options.knee !== undefined) ? options.knee : (_comp.knee !== undefined ? _comp.knee : 30));
        self.ratio = _createAudioParam((options && options.ratio !== undefined) ? options.ratio : (_comp.ratio !== undefined ? _comp.ratio : 12));
        self.attack = _createAudioParam((options && options.attack !== undefined) ? options.attack : (_comp.attack !== undefined ? _comp.attack : 0.003));
        self.release = _createAudioParam((options && options.release !== undefined) ? options.release : (_comp.release !== undefined ? _comp.release : 0.25));
        self.reduction = 0;
    }
    function _createDynamicsCompressorNode(ctx, options) {
        var node = Object.create(DynamicsCompressorNode.prototype);
        _initDynamicsCompressorNode(node, ctx, options);
        return node;
    }
    DynamicsCompressorNode.prototype = Object.create(AudioNode.prototype);

    // AnalyserNode
    function AnalyserNode(ctx, options) {
        throw new TypeError('Illegal constructor');
        _initAnalyserNode(this, ctx, options);
    }
    function _initAnalyserNode(self, ctx, options) {
        AudioNode.call(self, ctx);
        self.fftSize = (options && options.fftSize) || 2048;
        self.frequencyBinCount = self.fftSize / 2;
        self.minDecibels = -100;
        self.maxDecibels = -30;
        self.smoothingTimeConstant = 0.8;
    }
    function _createAnalyserNode(ctx, options) {
        var node = Object.create(AnalyserNode.prototype);
        _initAnalyserNode(node, ctx, options);
        return node;
    }
    AnalyserNode.prototype = Object.create(AudioNode.prototype);
    AnalyserNode.prototype.getFloatFrequencyData = function(arr) {};
    AnalyserNode.prototype.getByteFrequencyData = function(arr) {};
    AnalyserNode.prototype.getFloatTimeDomainData = function(arr) {};
    AnalyserNode.prototype.getByteTimeDomainData = function(arr) {};

    // GainNode
    function GainNode(ctx, options) {
        throw new TypeError('Illegal constructor');
        _initGainNode(this, ctx, options);
    }
    function _initGainNode(self, ctx, options) {
        AudioNode.call(self, ctx);
        self.gain = _createAudioParam((options && options.gain !== undefined) ? options.gain : 1);
    }
    function _createGainNode(ctx, options) {
        var node = Object.create(GainNode.prototype);
        _initGainNode(node, ctx, options);
        return node;
    }
    GainNode.prototype = Object.create(AudioNode.prototype);

    // AudioDestinationNode
    function AudioDestinationNode(ctx) {
        throw new TypeError('Illegal constructor');
        _initAudioDestinationNode(this, ctx);
    }
    function _initAudioDestinationNode(self, ctx) {
        _initAudioNode(self, ctx);
        self.maxChannelCount = 2;
        self.numberOfInputs = 1;
        self.numberOfOutputs = 0;
    }
    function _createAudioDestinationNode(ctx) {
        var node = Object.create(AudioDestinationNode.prototype);
        _initAudioDestinationNode(node, ctx);
        return node;
    }
    AudioDestinationNode.prototype = Object.create(AudioNode.prototype);

    // AudioBuffer stub
    function AudioBuffer(options) {
        throw new TypeError('Illegal constructor');
        _initAudioBuffer(this, options);
    }
    function _initAudioBuffer(self, options) {
        self.sampleRate = (options && options.sampleRate) || 44100;
        self.length = (options && options.length) || 0;
        self.duration = self.length / self.sampleRate;
        self.numberOfChannels = (options && options.numberOfChannels) || 1;
        self._data = new Float32Array(self.length);
    }
    function _createAudioBuffer(options) {
        var buf = Object.create(AudioBuffer.prototype);
        _initAudioBuffer(buf, options);
        return buf;
    }
    AudioBuffer.prototype.getChannelData = function(channel) {
        var data = new Float32Array(this.length);
        // Check for profile-driven fingerprint seed
        var fpSeed = _audioPrefs.channelDataSeed;
        if (typeof fpSeed === 'number' && fpSeed !== 0) {
            // Deterministic PRNG from seed (xorshift32)
            var s = fpSeed | 0;
            for (var i = 0; i < data.length; i++) {
                s = (s ^ (s << 13)) | 0;
                s = (s ^ (s >>> 17)) | 0;
                s = (s ^ (s << 5)) | 0;
                data[i] = ((s >>> 0) / 4294967296 - 0.5) * 0.0001;
            }
        } else {
            // Default: sinusoid fingerprint (deterministic, no seed)
            for (var i = 0; i < Math.min(data.length, 100); i++) {
                data[i] = Math.sin(i * 0.1) * 0.0001;
            }
        }
        return data;
    };
    AudioBuffer.prototype.copyFromChannel = function(dest, channel, offset) {};
    AudioBuffer.prototype.copyToChannel = function(src, channel, offset) {};

    // BaseAudioContext
    function BaseAudioContext(sampleRate) {
        throw new TypeError('Illegal constructor');
        _initBaseAudioContext(this, sampleRate);
    }
    function _initBaseAudioContext(self, sampleRate) {
        self._sampleRate = sampleRate || 44100;
        self._currentTime = 0;
        self._destination = _createAudioDestinationNode(self);
        self._listener = {};
        self._state = 'suspended';
        self._onstatechange = null;
    }
    Object.defineProperty(BaseAudioContext.prototype, 'sampleRate', { get: function() { return this._sampleRate; }, enumerable: true, configurable: true });
    Object.defineProperty(BaseAudioContext.prototype, 'currentTime', { get: function() { return this._currentTime; }, enumerable: true, configurable: true });
    Object.defineProperty(BaseAudioContext.prototype, 'destination', { get: function() { return this._destination; }, enumerable: true, configurable: true });
    Object.defineProperty(BaseAudioContext.prototype, 'listener', { get: function() { return this._listener; }, enumerable: true, configurable: true });
    Object.defineProperty(BaseAudioContext.prototype, 'state', { get: function() { return this._state; }, enumerable: true, configurable: true });
    Object.defineProperty(BaseAudioContext.prototype, 'onstatechange', { get: function() { return this._onstatechange; }, set: function(v) { this._onstatechange = v; }, enumerable: true, configurable: true });
    BaseAudioContext.prototype.createOscillator = function(options) {
        return _createOscillatorNode(this, options);
    };
    BaseAudioContext.prototype.createDynamicsCompressor = function(options) {
        return _createDynamicsCompressorNode(this, options);
    };
    BaseAudioContext.prototype.createAnalyser = function(options) {
        return _createAnalyserNode(this, options);
    };
    BaseAudioContext.prototype.createGain = function(options) {
        return _createGainNode(this, options);
    };
    BaseAudioContext.prototype.createBuffer = function(channels, length, sampleRate) {
        return _createAudioBuffer({ numberOfChannels: channels, length: length, sampleRate: sampleRate });
    };
    BaseAudioContext.prototype.createBufferSource = function() {
        var node = _createAudioNode(this);
        node.buffer = null;
        node.loop = false;
        node.loopStart = 0;
        node.loopEnd = 0;
        node.playbackRate = _createAudioParam(1);
        node.detune = _createAudioParam(0);
        node.onended = null;
        node.start = function() {};
        node.stop = function() {};
        return node;
    };
    BaseAudioContext.prototype.createScriptProcessor = function(bufferSize, inputChannels, outputChannels) {
        var node = _createAudioNode(this);
        node.bufferSize = bufferSize || 4096;
        node.onaudioprocess = null;
        return node;
    };
    BaseAudioContext.prototype.createChannelSplitter = function(n) {
        var node = _createAudioNode(this);
        node.numberOfOutputs = n || 6;
        return node;
    };
    BaseAudioContext.prototype.createChannelMerger = function(n) {
        var node = _createAudioNode(this);
        node.numberOfInputs = n || 6;
        return node;
    };
    BaseAudioContext.prototype.createConvolver = function() {
        var node = _createAudioNode(this);
        node.buffer = null;
        node.normalize = true;
        return node;
    };
    BaseAudioContext.prototype.createDelay = function(maxDelay) {
        var node = _createAudioNode(this);
        node.delayTime = _createAudioParam(0);
        return node;
    };
    BaseAudioContext.prototype.createBiquadFilter = function() {
        var node = _createAudioNode(this);
        node.type = 'lowpass';
        node.frequency = _createAudioParam(350);
        node.detune = _createAudioParam(0);
        node.Q = _createAudioParam(1);
        node.gain = _createAudioParam(0);
        node.getFrequencyResponse = function() {};
        return node;
    };
    BaseAudioContext.prototype.createWaveShaper = function() {
        var node = _createAudioNode(this);
        node.curve = null;
        node.oversample = 'none';
        return node;
    };
    BaseAudioContext.prototype.createStereoPanner = function() {
        var node = _createAudioNode(this);
        node.pan = _createAudioParam(0);
        return node;
    };
    BaseAudioContext.prototype.createPanner = function() {
        var node = _createAudioNode(this);
        node.panningModel = 'equalpower';
        node.distanceModel = 'inverse';
        node.positionX = _createAudioParam(0);
        node.positionY = _createAudioParam(0);
        node.positionZ = _createAudioParam(0);
        node.orientationX = _createAudioParam(1);
        node.orientationY = _createAudioParam(0);
        node.orientationZ = _createAudioParam(0);
        node.refDistance = 1;
        node.maxDistance = 10000;
        node.rolloffFactor = 1;
        node.coneInnerAngle = 360;
        node.coneOuterAngle = 0;
        node.coneOuterGain = 0;
        return node;
    };
    BaseAudioContext.prototype.decodeAudioData = function(buffer, successCb, errorCb) {
        var ab = _createAudioBuffer({ length: 1, sampleRate: this.sampleRate });
        if (successCb) { setTimeout(function() { successCb(ab); }, 0); return; }
        return Promise.resolve(ab);
    };
    BaseAudioContext.prototype.resume = function() { this._state = 'running'; return Promise.resolve(); };
    BaseAudioContext.prototype.suspend = function() { this._state = 'suspended'; return Promise.resolve(); };
    BaseAudioContext.prototype.close = function() { this._state = 'closed'; return Promise.resolve(); };
    BaseAudioContext.prototype.addEventListener = function() {};
    BaseAudioContext.prototype.removeEventListener = function() {};
    BaseAudioContext.prototype.dispatchEvent = function() { return true; };

    // AudioContext
    var _audioPrefs = (typeof globalThis.__iv8AudioPrefs === 'object' && globalThis.__iv8AudioPrefs) ? globalThis.__iv8AudioPrefs : {};
    function AudioContext(options) {
        _initBaseAudioContext(this, options && options.sampleRate);
        this._baseLatency = _audioPrefs.baseLatency || 0.005;
        this._outputLatency = _audioPrefs.outputLatency || 0.01;
    }
    AudioContext.prototype = Object.create(BaseAudioContext.prototype);
    AudioContext.prototype.constructor = AudioContext;
    Object.defineProperty(AudioContext.prototype, 'baseLatency', { get: function() { return this._baseLatency; }, enumerable: true, configurable: true });
    Object.defineProperty(AudioContext.prototype, 'outputLatency', { get: function() { return this._outputLatency; }, enumerable: true, configurable: true });
    AudioContext.prototype.getOutputTimestamp = function() {
        return { contextTime: this._currentTime, performanceTime: performance.now() };
    };
    AudioContext.prototype.createMediaStreamSource = function(stream) { return _createAudioNode(this); };
    AudioContext.prototype.createMediaStreamDestination = function() {
        var node = _createAudioNode(this);
        node.stream = { getTracks: function() { return []; }, getAudioTracks: function() { return []; } };
        return node;
    };
    AudioContext.prototype.createMediaElementSource = function(el) { return _createAudioNode(this); };

    // OfflineAudioContext
    function OfflineAudioContext(numberOfChannels, length, sampleRate) {
        if (typeof numberOfChannels === 'object') {
            var opts = numberOfChannels;
            numberOfChannels = opts.numberOfChannels || 1;
            length = opts.length || 44100;
            sampleRate = opts.sampleRate || 44100;
        }
        _initBaseAudioContext(this, sampleRate);
        this.length = length;
        this.numberOfChannels = numberOfChannels;
        this._buffer = _createAudioBuffer({ numberOfChannels: numberOfChannels, length: length, sampleRate: sampleRate });
    }
    OfflineAudioContext.prototype = Object.create(BaseAudioContext.prototype);
    OfflineAudioContext.prototype.constructor = OfflineAudioContext;
    OfflineAudioContext.prototype.startRendering = function() {
        var self = this;
        return Promise.resolve(self._buffer);
    };
    OfflineAudioContext.prototype.suspend = function(suspendTime) { return Promise.resolve(); };
    OfflineAudioContext.prototype.resume = function() { return Promise.resolve(); };

    // Install on globalThis
    globalThis.AudioContext = AudioContext;
    globalThis.webkitAudioContext = AudioContext;
    globalThis.OfflineAudioContext = OfflineAudioContext;
    globalThis.AudioBuffer = AudioBuffer;
    globalThis.AudioNode = AudioNode;
    globalThis.AudioParam = AudioParam;
    globalThis.GainNode = GainNode;
    globalThis.OscillatorNode = OscillatorNode;
    globalThis.AnalyserNode = AnalyserNode;
    globalThis.DynamicsCompressorNode = DynamicsCompressorNode;
    globalThis.BaseAudioContext = BaseAudioContext;
})();
"#;
