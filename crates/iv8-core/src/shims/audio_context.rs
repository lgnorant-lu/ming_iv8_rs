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
        this.value = value !== undefined ? value : 0;
        this.defaultValue = this.value;
        this.minValue = -3.4028234663852886e+38;
        this.maxValue = 3.4028234663852886e+38;
        this.automationRate = 'a-rate';
    }
    AudioParam.prototype.setValueAtTime = function(v, t) { this.value = v; return this; };
    AudioParam.prototype.linearRampToValueAtTime = function(v, t) { return this; };
    AudioParam.prototype.exponentialRampToValueAtTime = function(v, t) { return this; };
    AudioParam.prototype.setTargetAtTime = function(v, t, tc) { return this; };
    AudioParam.prototype.setValueCurveAtTime = function(vs, t, d) { return this; };
    AudioParam.prototype.cancelScheduledValues = function(t) { return this; };
    AudioParam.prototype.cancelAndHoldAtTime = function(t) { return this; };

    // AudioNode base
    function AudioNode(ctx) {
        this.context = ctx;
        this.numberOfInputs = 0;
        this.numberOfOutputs = 1;
        this.channelCount = 2;
        this.channelCountMode = 'max';
        this.channelInterpretation = 'speakers';
    }
    AudioNode.prototype.connect = function(dest) { return dest; };
    AudioNode.prototype.disconnect = function() {};
    AudioNode.prototype.addEventListener = function() {};
    AudioNode.prototype.removeEventListener = function() {};
    AudioNode.prototype.dispatchEvent = function() { return true; };

    // OscillatorNode
    function OscillatorNode(ctx, options) {
        AudioNode.call(this, ctx);
        this.type = (options && options.type) || 'sine';
        this.frequency = new AudioParam((options && options.frequency) || 440);
        this.detune = new AudioParam(0);
        this.onended = null;
    }
    OscillatorNode.prototype = Object.create(AudioNode.prototype);
    OscillatorNode.prototype.start = function(when) {};
    OscillatorNode.prototype.stop = function(when) {};

    // DynamicsCompressorNode
    function DynamicsCompressorNode(ctx, options) {
        AudioNode.call(this, ctx);
        this.threshold = new AudioParam((options && options.threshold !== undefined) ? options.threshold : -24);
        this.knee = new AudioParam((options && options.knee !== undefined) ? options.knee : 30);
        this.ratio = new AudioParam((options && options.ratio !== undefined) ? options.ratio : 12);
        this.attack = new AudioParam((options && options.attack !== undefined) ? options.attack : 0.003);
        this.release = new AudioParam((options && options.release !== undefined) ? options.release : 0.25);
        this.reduction = 0;
    }
    DynamicsCompressorNode.prototype = Object.create(AudioNode.prototype);

    // AnalyserNode
    function AnalyserNode(ctx, options) {
        AudioNode.call(this, ctx);
        this.fftSize = (options && options.fftSize) || 2048;
        this.frequencyBinCount = this.fftSize / 2;
        this.minDecibels = -100;
        this.maxDecibels = -30;
        this.smoothingTimeConstant = 0.8;
    }
    AnalyserNode.prototype = Object.create(AudioNode.prototype);
    AnalyserNode.prototype.getFloatFrequencyData = function(arr) {};
    AnalyserNode.prototype.getByteFrequencyData = function(arr) {};
    AnalyserNode.prototype.getFloatTimeDomainData = function(arr) {};
    AnalyserNode.prototype.getByteTimeDomainData = function(arr) {};

    // GainNode
    function GainNode(ctx, options) {
        AudioNode.call(this, ctx);
        this.gain = new AudioParam((options && options.gain !== undefined) ? options.gain : 1);
    }
    GainNode.prototype = Object.create(AudioNode.prototype);

    // AudioDestinationNode
    function AudioDestinationNode(ctx) {
        AudioNode.call(this, ctx);
        this.maxChannelCount = 2;
        this.numberOfInputs = 1;
        this.numberOfOutputs = 0;
    }
    AudioDestinationNode.prototype = Object.create(AudioNode.prototype);

    // AudioBuffer stub
    function AudioBuffer(options) {
        this.sampleRate = (options && options.sampleRate) || 44100;
        this.length = (options && options.length) || 0;
        this.duration = this.length / this.sampleRate;
        this.numberOfChannels = (options && options.numberOfChannels) || 1;
        this._data = new Float32Array(this.length);
    }
    AudioBuffer.prototype.getChannelData = function(channel) {
        var data = new Float32Array(this.length);
        for (var i = 0; i < Math.min(data.length, 100); i++) {
            data[i] = Math.sin(i * 0.1) * 0.0001;
        }
        return data;
    };
    AudioBuffer.prototype.copyFromChannel = function(dest, channel, offset) {};
    AudioBuffer.prototype.copyToChannel = function(src, channel, offset) {};

    // BaseAudioContext
    function BaseAudioContext(sampleRate) {
        this.sampleRate = sampleRate || 44100;
        this.currentTime = 0;
        this.destination = new AudioDestinationNode(this);
        this.listener = {};
        this.state = 'suspended';
        this.onstatechange = null;
    }
    BaseAudioContext.prototype.createOscillator = function(options) {
        return new OscillatorNode(this, options);
    };
    BaseAudioContext.prototype.createDynamicsCompressor = function(options) {
        return new DynamicsCompressorNode(this, options);
    };
    BaseAudioContext.prototype.createAnalyser = function(options) {
        return new AnalyserNode(this, options);
    };
    BaseAudioContext.prototype.createGain = function(options) {
        return new GainNode(this, options);
    };
    BaseAudioContext.prototype.createBuffer = function(channels, length, sampleRate) {
        return new AudioBuffer({ numberOfChannels: channels, length: length, sampleRate: sampleRate });
    };
    BaseAudioContext.prototype.createBufferSource = function() {
        var node = new AudioNode(this);
        node.buffer = null;
        node.loop = false;
        node.loopStart = 0;
        node.loopEnd = 0;
        node.playbackRate = new AudioParam(1);
        node.detune = new AudioParam(0);
        node.onended = null;
        node.start = function() {};
        node.stop = function() {};
        return node;
    };
    BaseAudioContext.prototype.createScriptProcessor = function(bufferSize, inputChannels, outputChannels) {
        var node = new AudioNode(this);
        node.bufferSize = bufferSize || 4096;
        node.onaudioprocess = null;
        return node;
    };
    BaseAudioContext.prototype.createChannelSplitter = function(n) {
        var node = new AudioNode(this);
        node.numberOfOutputs = n || 6;
        return node;
    };
    BaseAudioContext.prototype.createChannelMerger = function(n) {
        var node = new AudioNode(this);
        node.numberOfInputs = n || 6;
        return node;
    };
    BaseAudioContext.prototype.createConvolver = function() {
        var node = new AudioNode(this);
        node.buffer = null;
        node.normalize = true;
        return node;
    };
    BaseAudioContext.prototype.createDelay = function(maxDelay) {
        var node = new AudioNode(this);
        node.delayTime = new AudioParam(0);
        return node;
    };
    BaseAudioContext.prototype.createBiquadFilter = function() {
        var node = new AudioNode(this);
        node.type = 'lowpass';
        node.frequency = new AudioParam(350);
        node.detune = new AudioParam(0);
        node.Q = new AudioParam(1);
        node.gain = new AudioParam(0);
        node.getFrequencyResponse = function() {};
        return node;
    };
    BaseAudioContext.prototype.createWaveShaper = function() {
        var node = new AudioNode(this);
        node.curve = null;
        node.oversample = 'none';
        return node;
    };
    BaseAudioContext.prototype.createStereoPanner = function() {
        var node = new AudioNode(this);
        node.pan = new AudioParam(0);
        return node;
    };
    BaseAudioContext.prototype.createPanner = function() {
        var node = new AudioNode(this);
        node.panningModel = 'equalpower';
        node.distanceModel = 'inverse';
        node.positionX = new AudioParam(0);
        node.positionY = new AudioParam(0);
        node.positionZ = new AudioParam(0);
        node.orientationX = new AudioParam(1);
        node.orientationY = new AudioParam(0);
        node.orientationZ = new AudioParam(0);
        node.refDistance = 1;
        node.maxDistance = 10000;
        node.rolloffFactor = 1;
        node.coneInnerAngle = 360;
        node.coneOuterAngle = 0;
        node.coneOuterGain = 0;
        return node;
    };
    BaseAudioContext.prototype.decodeAudioData = function(buffer, successCb, errorCb) {
        var ab = new AudioBuffer({ length: 1, sampleRate: this.sampleRate });
        if (successCb) { setTimeout(function() { successCb(ab); }, 0); return; }
        return Promise.resolve(ab);
    };
    BaseAudioContext.prototype.resume = function() { this.state = 'running'; return Promise.resolve(); };
    BaseAudioContext.prototype.suspend = function() { this.state = 'suspended'; return Promise.resolve(); };
    BaseAudioContext.prototype.close = function() { this.state = 'closed'; return Promise.resolve(); };
    BaseAudioContext.prototype.addEventListener = function() {};
    BaseAudioContext.prototype.removeEventListener = function() {};
    BaseAudioContext.prototype.dispatchEvent = function() { return true; };

    // AudioContext
    function AudioContext(options) {
        BaseAudioContext.call(this, options && options.sampleRate);
        this.baseLatency = 0.005;
        this.outputLatency = 0.01;
    }
    AudioContext.prototype = Object.create(BaseAudioContext.prototype);
    AudioContext.prototype.constructor = AudioContext;
    AudioContext.prototype.getOutputTimestamp = function() {
        return { contextTime: this.currentTime, performanceTime: performance.now() };
    };
    AudioContext.prototype.createMediaStreamSource = function(stream) { return new AudioNode(this); };
    AudioContext.prototype.createMediaStreamDestination = function() {
        var node = new AudioNode(this);
        node.stream = { getTracks: function() { return []; }, getAudioTracks: function() { return []; } };
        return node;
    };
    AudioContext.prototype.createMediaElementSource = function(el) { return new AudioNode(this); };

    // OfflineAudioContext
    function OfflineAudioContext(numberOfChannels, length, sampleRate) {
        if (typeof numberOfChannels === 'object') {
            var opts = numberOfChannels;
            numberOfChannels = opts.numberOfChannels || 1;
            length = opts.length || 44100;
            sampleRate = opts.sampleRate || 44100;
        }
        BaseAudioContext.call(this, sampleRate);
        this.length = length;
        this.numberOfChannels = numberOfChannels;
        this._buffer = new AudioBuffer({ numberOfChannels: numberOfChannels, length: length, sampleRate: sampleRate });
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
