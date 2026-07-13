//! AudioContext / OfflineAudioContext / Web Audio API stubs.
//!
//! FingerprintJS uses AudioContext for audio fingerprinting.
//! Extracted from document_props.rs for code organization.
//!
//! Dependencies: performance.now(), setTimeout (events/timers.rs)
//!
//! North Star Phase 1 (v0.8.90): shim preserves codegen prototype.
//! Instead of creating new constructors with `Object.create()`, the shim
//! wraps the codegen constructors and installs JS methods on the codegen
//! prototype. This keeps Symbol.toStringTag, instanceof, and prototype chain
//! intact without post-hoc fixes (TO_STRING_TAG_FIX_JS).

pub const AUDIO_CONTEXT_JS: &str = r#"
(function() {
    if (globalThis.__iv8AudioShimInstalled) return;
    globalThis.__iv8AudioShimInstalled = true;
    var CodegenAudioContext = globalThis.AudioContext;
    var CodegenOfflineAudioContext = globalThis.OfflineAudioContext;
    var CodegenAudioBuffer = globalThis.AudioBuffer;
    var CodegenAudioNode = globalThis.AudioNode;
    var CodegenAudioParam = globalThis.AudioParam;
    var CodegenGainNode = globalThis.GainNode;
    var CodegenOscillatorNode = globalThis.OscillatorNode;
    var CodegenAnalyserNode = globalThis.AnalyserNode;
    var CodegenDynamicsCompressorNode = globalThis.DynamicsCompressorNode;
    var CodegenBaseAudioContext = globalThis.BaseAudioContext;
    var CodegenAudioDestinationNode = globalThis.AudioDestinationNode;

    var AudioParamProto = CodegenAudioParam.prototype;
    var AudioNodeProto = CodegenAudioNode.prototype;
    var OscillatorNodeProto = CodegenOscillatorNode.prototype;
    var DynamicsCompressorNodeProto = CodegenDynamicsCompressorNode.prototype;
    var AnalyserNodeProto = CodegenAnalyserNode.prototype;
    var GainNodeProto = CodegenGainNode.prototype;
    var AudioDestinationNodeProto = CodegenAudioDestinationNode.prototype;
    var AudioBufferProto = CodegenAudioBuffer.prototype;
    var BaseAudioContextProto = CodegenBaseAudioContext.prototype;
    var AudioContextProto = CodegenAudioContext.prototype;
    var OfflineAudioContextProto = CodegenOfflineAudioContext.prototype;

    var _audioPrefs = (typeof globalThis.__iv8AudioPrefs === 'object' && globalThis.__iv8AudioPrefs) ? globalThis.__iv8AudioPrefs : {};
    var _defaultSampleRate = (_audioPrefs.sampleRate !== undefined) ? _audioPrefs.sampleRate : 48000;

    function _override(proto, name, getter, setter) {
        Object.defineProperty(proto, name, {
            get: getter, set: setter,
            enumerable: true, configurable: true
        });
    }
    function _overrideValue(proto, name) {
        var slot = '_' + name;
        _override(proto, name,
            function() { return this[slot]; },
            function(v) {
                Object.defineProperty(this, slot, {
                    value: v, writable: true, enumerable: false, configurable: true
                });
            }
        );
    }
    function _setSlot(obj, name, val) {
        Object.defineProperty(obj, '_' + name, {
            value: val, writable: true, enumerable: false, configurable: true
        });
    }

    // AudioParam stub
    function AudioParam(value) {
        throw new TypeError('Illegal constructor');
        _initAudioParam(this, value);
    }
    function _initAudioParam(self, value) {
        _setSlot(self, '_value'.substring(1), value !== undefined ? value : 0);
        _setSlot(self, '_defaultValue'.substring(1), self._value);
        _setSlot(self, '_minValue'.substring(1), -3.4028234663852886e+38);
        _setSlot(self, '_maxValue'.substring(1), 3.4028234663852886e+38);
        _setSlot(self, '_automationRate'.substring(1), 'a-rate');
    }
    AudioParam.prototype = AudioParamProto;
    Object.defineProperty(AudioParam.prototype, 'constructor', {value: AudioParam, writable: true, enumerable: false, configurable: true});
    _overrideValue(AudioParamProto, 'value');
    _overrideValue(AudioParamProto, 'defaultValue');
    _overrideValue(AudioParamProto, 'minValue');
    _overrideValue(AudioParamProto, 'maxValue');
    _overrideValue(AudioParamProto, 'automationRate');
    AudioParamProto.setValueAtTime = function(v, t) { this.value = v; return this; };
    AudioParamProto.linearRampToValueAtTime = function(v, t) { return this; };
    AudioParamProto.exponentialRampToValueAtTime = function(v, t) { return this; };
    AudioParamProto.setTargetAtTime = function(v, t, tc) { return this; };
    AudioParamProto.setValueCurveAtTime = function(vs, t, d) { return this; };
    AudioParamProto.cancelScheduledValues = function(t) { return this; };
    AudioParamProto.cancelAndHoldAtTime = function(t) { return this; };

    // Internal factory: creates AudioParam without requiring `new` (bypasses
    // the illegal-constructor guard). Used by node constructors and factory
    // methods so external `new AudioParam()` throws but internal creation works.
    function _createAudioParam(value) {
        var p = Object.create(CodegenAudioParam.prototype);
        _initAudioParam(p, value);
        return p;
    }

    // AudioNode base
    function AudioNode(ctx) {
        throw new TypeError('Illegal constructor');
        _initAudioNode(this, ctx);
    }
    function _initAudioNode(self, ctx) {
        _setSlot(self, '_context'.substring(1), ctx);
        _setSlot(self, '_numberOfInputs'.substring(1), 0);
        _setSlot(self, '_numberOfOutputs'.substring(1), 1);
        _setSlot(self, '_channelCount'.substring(1), 2);
        _setSlot(self, '_channelCountMode'.substring(1), 'max');
        _setSlot(self, '_channelInterpretation'.substring(1), 'speakers');
    }
    AudioNode.prototype = AudioNodeProto;
    Object.defineProperty(AudioNode.prototype, 'constructor', {value: AudioNode, writable: true, enumerable: false, configurable: true});
    _overrideValue(AudioNodeProto, 'context');
    _overrideValue(AudioNodeProto, 'numberOfInputs');
    _overrideValue(AudioNodeProto, 'numberOfOutputs');
    _overrideValue(AudioNodeProto, 'channelCount');
    _overrideValue(AudioNodeProto, 'channelCountMode');
    _overrideValue(AudioNodeProto, 'channelInterpretation');
    AudioNodeProto.connect = function(dest) { return dest; };
    AudioNodeProto.disconnect = function() {};
    AudioNodeProto.addEventListener = function() {};
    AudioNodeProto.removeEventListener = function() {};
    AudioNodeProto.dispatchEvent = function() { return true; };
    function _createAudioNode(ctx) {
        var node = Object.create(CodegenAudioNode.prototype);
        _initAudioNode(node, ctx);
        return node;
    }

    // OscillatorNode
    function OscillatorNode(ctx, options) {
        throw new TypeError('Illegal constructor');
        _initOscillatorNode(this, ctx, options);
    }
    function _initOscillatorNode(self, ctx, options) {
        _initAudioNode(self, ctx);
        _setSlot(self, '_type'.substring(1), (options && options.type) || 'sine');
        _setSlot(self, '_frequency'.substring(1), _createAudioParam((options && options.frequency) || 440));
        _setSlot(self, '_detune'.substring(1), _createAudioParam(0));
        _setSlot(self, '_onended'.substring(1), null);
    }
    OscillatorNode.prototype = OscillatorNodeProto;
    Object.defineProperty(OscillatorNode.prototype, 'constructor', {value: OscillatorNode, writable: true, enumerable: false, configurable: true});
    Object.setPrototypeOf(OscillatorNodeProto, AudioNodeProto);
    _overrideValue(OscillatorNodeProto, 'type');
    _overrideValue(OscillatorNodeProto, 'frequency');
    _overrideValue(OscillatorNodeProto, 'detune');
    _overrideValue(OscillatorNodeProto, 'onended');
    OscillatorNodeProto.start = function(when) {};
    OscillatorNodeProto.stop = function(when) {};
    function _createOscillatorNode(ctx, options) {
        var node = Object.create(CodegenOscillatorNode.prototype);
        _initOscillatorNode(node, ctx, options);
        return node;
    }

    // DynamicsCompressorNode
    function DynamicsCompressorNode(ctx, options) {
        throw new TypeError('Illegal constructor');
        _initDynamicsCompressorNode(this, ctx, options);
    }
    function _initDynamicsCompressorNode(self, ctx, options) {
        _initAudioNode(self, ctx);
        var _comp = (_audioPrefs && _audioPrefs.compressor) ? _audioPrefs.compressor : {};
        _setSlot(self, '_threshold'.substring(1), _createAudioParam((options && options.threshold !== undefined) ? options.threshold : (_comp.threshold !== undefined ? _comp.threshold : -24)));
        _setSlot(self, '_knee'.substring(1), _createAudioParam((options && options.knee !== undefined) ? options.knee : (_comp.knee !== undefined ? _comp.knee : 30)));
        _setSlot(self, '_ratio'.substring(1), _createAudioParam((options && options.ratio !== undefined) ? options.ratio : (_comp.ratio !== undefined ? _comp.ratio : 12)));
        _setSlot(self, '_attack'.substring(1), _createAudioParam((options && options.attack !== undefined) ? options.attack : (_comp.attack !== undefined ? _comp.attack : 0.003)));
        _setSlot(self, '_release'.substring(1), _createAudioParam((options && options.release !== undefined) ? options.release : (_comp.release !== undefined ? _comp.release : 0.25)));
        _setSlot(self, '_reduction'.substring(1), (_comp.reduction !== undefined) ? _comp.reduction : 0);
    }
    DynamicsCompressorNode.prototype = DynamicsCompressorNodeProto;
    Object.defineProperty(DynamicsCompressorNode.prototype, 'constructor', {value: DynamicsCompressorNode, writable: true, enumerable: false, configurable: true});
    Object.setPrototypeOf(DynamicsCompressorNodeProto, AudioNodeProto);
    _overrideValue(DynamicsCompressorNodeProto, 'threshold');
    _overrideValue(DynamicsCompressorNodeProto, 'knee');
    _overrideValue(DynamicsCompressorNodeProto, 'ratio');
    _overrideValue(DynamicsCompressorNodeProto, 'attack');
    _overrideValue(DynamicsCompressorNodeProto, 'release');
    _overrideValue(DynamicsCompressorNodeProto, 'reduction');
    function _createDynamicsCompressorNode(ctx, options) {
        var node = Object.create(CodegenDynamicsCompressorNode.prototype);
        _initDynamicsCompressorNode(node, ctx, options);
        return node;
    }

    // AnalyserNode
    function AnalyserNode(ctx, options) {
        throw new TypeError('Illegal constructor');
        _initAnalyserNode(this, ctx, options);
    }
    function _initAnalyserNode(self, ctx, options) {
        _initAudioNode(self, ctx);
        _setSlot(self, '_fftSize'.substring(1), (options && options.fftSize) || 2048);
        _setSlot(self, '_frequencyBinCount'.substring(1), self._fftSize / 2);
        _setSlot(self, '_minDecibels'.substring(1), -100);
        _setSlot(self, '_maxDecibels'.substring(1), -30);
        _setSlot(self, '_smoothingTimeConstant'.substring(1), 0.8);
    }
    AnalyserNode.prototype = AnalyserNodeProto;
    Object.defineProperty(AnalyserNode.prototype, 'constructor', {value: AnalyserNode, writable: true, enumerable: false, configurable: true});
    Object.setPrototypeOf(AnalyserNodeProto, AudioNodeProto);
    _overrideValue(AnalyserNodeProto, 'fftSize');
    _overrideValue(AnalyserNodeProto, 'frequencyBinCount');
    _overrideValue(AnalyserNodeProto, 'minDecibels');
    _overrideValue(AnalyserNodeProto, 'maxDecibels');
    _overrideValue(AnalyserNodeProto, 'smoothingTimeConstant');
    AnalyserNodeProto.getFloatFrequencyData = function(arr) {
        var n = arr.length;
        var prefs = _audioPrefs.analyserData;
        if (prefs && prefs.floatFrequency) {
            var src = prefs.floatFrequency;
            if (typeof src === 'string') {
                try { src = JSON.parse(src); } catch (e) { src = null; }
            }
            if (Array.isArray(src)) {
                for (var i = 0; i < n; i++) { arr[i] = src[i] !== undefined ? src[i] : -Infinity; }
                return;
            }
        }
        for (var i = 0; i < n; i++) { arr[i] = -Infinity; }
    };
    AnalyserNodeProto.getByteFrequencyData = function(arr) {
        var n = arr.length;
        var prefs = _audioPrefs.analyserData;
        if (prefs && prefs.byteFrequency) {
            var src = prefs.byteFrequency;
            if (typeof src === 'string') {
                try { src = JSON.parse(src); } catch (e) { src = null; }
            }
            if (Array.isArray(src)) {
                for (var i = 0; i < n; i++) { arr[i] = src[i] !== undefined ? (src[i] & 0xFF) : 0; }
                return;
            }
        }
        for (var i = 0; i < n; i++) { arr[i] = 0; }
    };
    AnalyserNodeProto.getFloatTimeDomainData = function(arr) {
        var n = arr.length;
        var prefs = _audioPrefs.analyserData;
        if (prefs && prefs.floatTimeDomain) {
            var src = prefs.floatTimeDomain;
            if (typeof src === 'string') {
                try { src = JSON.parse(src); } catch (e) { src = null; }
            }
            if (Array.isArray(src)) {
                for (var i = 0; i < n; i++) { arr[i] = src[i] !== undefined ? src[i] : 0; }
                return;
            }
        }
        for (var i = 0; i < n; i++) { arr[i] = 0; }
    };
    AnalyserNodeProto.getByteTimeDomainData = function(arr) {
        var n = arr.length;
        for (var i = 0; i < n; i++) { arr[i] = 128; }
    };
    function _createAnalyserNode(ctx, options) {
        var node = Object.create(CodegenAnalyserNode.prototype);
        _initAnalyserNode(node, ctx, options);
        return node;
    }

    // GainNode
    function GainNode(ctx, options) {
        throw new TypeError('Illegal constructor');
        _initGainNode(this, ctx, options);
    }
    function _initGainNode(self, ctx, options) {
        _initAudioNode(self, ctx);
        _setSlot(self, '_gain'.substring(1), _createAudioParam((options && options.gain !== undefined) ? options.gain : 1));
    }
    GainNode.prototype = GainNodeProto;
    Object.defineProperty(GainNode.prototype, 'constructor', {value: GainNode, writable: true, enumerable: false, configurable: true});
    Object.setPrototypeOf(GainNodeProto, AudioNodeProto);
    _overrideValue(GainNodeProto, 'gain');
    function _createGainNode(ctx, options) {
        var node = Object.create(CodegenGainNode.prototype);
        _initGainNode(node, ctx, options);
        return node;
    }

    // AudioDestinationNode
    function AudioDestinationNode(ctx) {
        throw new TypeError('Illegal constructor');
        _initAudioDestinationNode(this, ctx);
    }
    function _initAudioDestinationNode(self, ctx) {
        _initAudioNode(self, ctx);
        _setSlot(self, '_maxChannelCount'.substring(1), 2);
        _setSlot(self, '_numberOfInputs'.substring(1), 1);
        _setSlot(self, '_numberOfOutputs'.substring(1), 0);
    }
    AudioDestinationNode.prototype = AudioDestinationNodeProto;
    Object.defineProperty(AudioDestinationNode.prototype, 'constructor', {value: AudioDestinationNode, writable: true, enumerable: false, configurable: true});
    Object.setPrototypeOf(AudioDestinationNodeProto, AudioNodeProto);
    _overrideValue(AudioDestinationNodeProto, 'maxChannelCount');
    function _createAudioDestinationNode(ctx) {
        var node = Object.create(CodegenAudioDestinationNode.prototype);
        _initAudioDestinationNode(node, ctx);
        return node;
    }

    // AudioBuffer stub
    function AudioBuffer(options) {
        throw new TypeError('Illegal constructor');
        _initAudioBuffer(this, options);
    }
    function _initAudioBuffer(self, options) {
        _setSlot(self, '_sampleRate'.substring(1), (options && options.sampleRate) || _defaultSampleRate);
        _setSlot(self, '_length'.substring(1), (options && options.length) || 0);
        _setSlot(self, '_duration'.substring(1), self._length / self._sampleRate);
        _setSlot(self, '_numberOfChannels'.substring(1), (options && options.numberOfChannels) || 1);
        _setSlot(self, '_data'.substring(1), new Float32Array(self._length));
    }
    AudioBuffer.prototype = AudioBufferProto;
    Object.defineProperty(AudioBuffer.prototype, 'constructor', {value: AudioBuffer, writable: true, enumerable: false, configurable: true});
    _overrideValue(AudioBufferProto, 'sampleRate');
    _overrideValue(AudioBufferProto, 'length');
    _overrideValue(AudioBufferProto, 'duration');
    _overrideValue(AudioBufferProto, 'numberOfChannels');
    function _createAudioBuffer(options) {
        var buf = Object.create(CodegenAudioBuffer.prototype);
        _initAudioBuffer(buf, options);
        return buf;
    }
    AudioBufferProto.getChannelData = function(channel) {
        var data = new Float32Array(this._length);
        // Profile-driven fingerprint injection: captured channel data
        var fpData = _audioPrefs.channelData;
        if (fpData) {
            var src = null;
            if (typeof fpData === 'string') {
                // Try JSON array first
                try {
                    var parsed = JSON.parse(fpData);
                    if (Array.isArray(parsed)) { src = parsed; }
                } catch (e) {
                    // Not JSON, try base64
                    if (typeof atob === 'function') {
                        try {
                            var bin = atob(fpData);
                            var view = new DataView(new ArrayBuffer(bin.length));
                            for (var j = 0; j < bin.length; j++) { view.setUint8(j, bin.charCodeAt(j)); }
                            src = [];
                            for (var k = 0; k + 3 < bin.length; k += 4) {
                                src.push(view.getFloat32(k, true));
                            }
                        } catch (e2) { src = null; }
                    }
                }
            } else if (Array.isArray(fpData)) {
                src = fpData;
            }
            if (src) {
                var n = Math.min(src.length, data.length);
                for (var i = 0; i < n; i++) { data[i] = src[i]; }
                return data;
            }
        }
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
    AudioBufferProto.copyFromChannel = function(dest, channel, offset) {};
    AudioBufferProto.copyToChannel = function(src, channel, offset) {};

    // BaseAudioContext
    function BaseAudioContext(sampleRate) {
        throw new TypeError('Illegal constructor');
        _initBaseAudioContext(this, sampleRate);
    }
    function _initBaseAudioContext(self, sampleRate) {
        _setSlot(self, '_sampleRate'.substring(1), sampleRate || _defaultSampleRate);
        _setSlot(self, '_currentTime'.substring(1), 0);
        _setSlot(self, '_destination'.substring(1), _createAudioDestinationNode(self));
        _setSlot(self, '_listener'.substring(1), {});
        _setSlot(self, '_state'.substring(1), 'suspended');
        _setSlot(self, '_onstatechange'.substring(1), null);
    }
    BaseAudioContext.prototype = BaseAudioContextProto;
    Object.defineProperty(BaseAudioContext.prototype, 'constructor', {value: BaseAudioContext, writable: true, enumerable: false, configurable: true});
    _overrideValue(BaseAudioContextProto, 'sampleRate');
    _overrideValue(BaseAudioContextProto, 'currentTime');
    _overrideValue(BaseAudioContextProto, 'destination');
    _overrideValue(BaseAudioContextProto, 'listener');
    _overrideValue(BaseAudioContextProto, 'state');
    _overrideValue(BaseAudioContextProto, 'onstatechange');
    BaseAudioContextProto.createOscillator = function(options) {
        return _createOscillatorNode(this, options);
    };
    BaseAudioContextProto.createDynamicsCompressor = function(options) {
        return _createDynamicsCompressorNode(this, options);
    };
    BaseAudioContextProto.createAnalyser = function(options) {
        return _createAnalyserNode(this, options);
    };
    BaseAudioContextProto.createGain = function(options) {
        return _createGainNode(this, options);
    };
    BaseAudioContextProto.createBuffer = function(channels, length, sampleRate) {
        return _createAudioBuffer({ numberOfChannels: channels, length: length, sampleRate: sampleRate });
    };
    BaseAudioContextProto.createBufferSource = function() {
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
    BaseAudioContextProto.createScriptProcessor = function(bufferSize, inputChannels, outputChannels) {
        var node = _createAudioNode(this);
        node.bufferSize = bufferSize || 4096;
        node.onaudioprocess = null;
        return node;
    };
    BaseAudioContextProto.createChannelSplitter = function(n) {
        var node = _createAudioNode(this);
        node.numberOfOutputs = n || 6;
        return node;
    };
    BaseAudioContextProto.createChannelMerger = function(n) {
        var node = _createAudioNode(this);
        node.numberOfInputs = n || 6;
        return node;
    };
    BaseAudioContextProto.createConvolver = function() {
        var node = _createAudioNode(this);
        node.buffer = null;
        node.normalize = true;
        return node;
    };
    BaseAudioContextProto.createDelay = function(maxDelay) {
        var node = _createAudioNode(this);
        node.delayTime = _createAudioParam(0);
        return node;
    };
    BaseAudioContextProto.createBiquadFilter = function() {
        var node = _createAudioNode(this);
        node.type = 'lowpass';
        node.frequency = _createAudioParam(350);
        node.detune = _createAudioParam(0);
        node.Q = _createAudioParam(1);
        node.gain = _createAudioParam(0);
        node.getFrequencyResponse = function() {};
        return node;
    };
    BaseAudioContextProto.createWaveShaper = function() {
        var node = _createAudioNode(this);
        node.curve = null;
        node.oversample = 'none';
        return node;
    };
    BaseAudioContextProto.createStereoPanner = function() {
        var node = _createAudioNode(this);
        node.pan = _createAudioParam(0);
        return node;
    };
    BaseAudioContextProto.createPanner = function() {
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
    BaseAudioContextProto.decodeAudioData = function(buffer, successCb, errorCb) {
        var ab = _createAudioBuffer({ length: 1, sampleRate: this.sampleRate });
        if (successCb) { setTimeout(function() { successCb(ab); }, 0); return; }
        return Promise.resolve(ab);
    };
    BaseAudioContextProto.resume = function() { this._state = 'running'; return Promise.resolve(); };
    BaseAudioContextProto.suspend = function() { this._state = 'suspended'; return Promise.resolve(); };
    BaseAudioContextProto.close = function() { this._state = 'closed'; return Promise.resolve(); };
    BaseAudioContextProto.addEventListener = function() {};
    BaseAudioContextProto.removeEventListener = function() {};
    BaseAudioContextProto.dispatchEvent = function() { return true; };

    // AudioContext
    function AudioContext(options) {
        if (!(this instanceof AudioContext)) {
            throw new TypeError("Failed to construct 'AudioContext': Please use the 'new' operator");
        }
        var inst = Reflect.construct(CodegenAudioContext, [], new.target || AudioContext);
        _initBaseAudioContext(inst, (options && options.sampleRate) || _defaultSampleRate);
        _setSlot(inst, 'baseLatency', _audioPrefs.baseLatency != null ? _audioPrefs.baseLatency : 0.05);
        _setSlot(inst, 'outputLatency', _audioPrefs.outputLatency != null ? _audioPrefs.outputLatency : 0);
        inst.__iv8BaseLatency = _audioPrefs.baseLatency != null ? _audioPrefs.baseLatency : 0.05;
        inst.__iv8OutputLatency = _audioPrefs.outputLatency != null ? _audioPrefs.outputLatency : 0;
        inst.__iv8SampleRate = (options && options.sampleRate) || _defaultSampleRate;
        inst.__iv8State = 'suspended';
        inst.__iv8Destination = _createAudioDestinationNode(inst, 2);
        inst.__iv8CurrentTime = 0;
        inst.__iv8Listener = {};
        inst.__iv8Onstatechange = null;
        return inst;
    }
    AudioContext.prototype = AudioContextProto;
    Object.defineProperty(AudioContext.prototype, 'constructor', {value: AudioContext, writable: true, enumerable: false, configurable: true});
    Object.defineProperty(AudioContext, 'prototype', {writable: false, enumerable: false, configurable: false});
    Object.setPrototypeOf(AudioContextProto, BaseAudioContextProto);
    _overrideValue(AudioContextProto, 'baseLatency');
    _overrideValue(AudioContextProto, 'outputLatency');
    AudioContextProto.getOutputTimestamp = function() {
        return { contextTime: this._currentTime, performanceTime: performance.now() };
    };
    AudioContextProto.createMediaStreamSource = function(stream) { return _createAudioNode(this); };
    AudioContextProto.createMediaStreamDestination = function() {
        var node = _createAudioNode(this);
        node.stream = { getTracks: function() { return []; }, getAudioTracks: function() { return []; } };
        return node;
    };
    AudioContextProto.createMediaElementSource = function(el) { return _createAudioNode(this); };

    AudioContextProto.audioWorklet = undefined;
    AudioContextProto.createConstantSource = function() {
        var node = _createAudioNode(this);
        node.offset = _createAudioParam(0);
        return node;
    };
    AudioContextProto.createIIRFilter = function(feedforward, feedback) {
        var node = _createAudioNode(this);
        return node;
    };
    AudioContextProto.createPeriodicWave = function(real, imag, constraints) {
        var wave = { _real: real, _imag: imag };
        return wave;
    };
    Object.defineProperty(AudioContextProto, 'onerror', { get: function() { return this._onerror || null; }, set: function(v) { this._onerror = v; }, enumerable: true, configurable: true });
    Object.defineProperty(AudioContextProto, 'onsinkchange', { get: function() { return this._onsinkchange || null; }, set: function(v) { this._onsinkchange = v; }, enumerable: true, configurable: true });
    Object.defineProperty(AudioContextProto, 'playbackStats', { get: function() { return {}; }, enumerable: true, configurable: true });
    AudioContextProto.setSinkId = function(sinkId) { return Promise.resolve(); };
    Object.defineProperty(AudioContextProto, 'sinkId', { get: function() { return ''; }, enumerable: true, configurable: true });

    // OfflineAudioContext
    function OfflineAudioContext(numberOfChannels, length, sampleRate) {
        if (!(this instanceof OfflineAudioContext)) {
            throw new TypeError("Failed to construct 'OfflineAudioContext': Please use the 'new' operator");
        }
        if (typeof numberOfChannels === 'object') {
            var opts = numberOfChannels;
            numberOfChannels = opts.numberOfChannels || 1;
            length = opts.length || 44100;
            sampleRate = opts.sampleRate || _defaultSampleRate;
        }
        sampleRate = sampleRate || _defaultSampleRate;
        var inst = Reflect.construct(CodegenOfflineAudioContext, [], new.target || OfflineAudioContext);
        _initBaseAudioContext(inst, sampleRate);
        _setSlot(inst, 'length', length);
        _setSlot(inst, 'numberOfChannels', numberOfChannels);
        _setSlot(inst, 'buffer', _createAudioBuffer({ numberOfChannels: numberOfChannels, length: length, sampleRate: sampleRate });
        return inst;
    }
    OfflineAudioContext.prototype = OfflineAudioContextProto;
    Object.defineProperty(OfflineAudioContext.prototype, 'constructor', {value: OfflineAudioContext, writable: true, enumerable: false, configurable: true});
    Object.defineProperty(OfflineAudioContext, 'prototype', {writable: false, enumerable: false, configurable: false});
    Object.setPrototypeOf(OfflineAudioContextProto, BaseAudioContextProto);
    _overrideValue(OfflineAudioContextProto, 'length');
    _overrideValue(OfflineAudioContextProto, 'numberOfChannels');
    OfflineAudioContextProto.startRendering = function() {
        var self = this;
        return Promise.resolve(self._buffer);
    };
    OfflineAudioContextProto.suspend = function(suspendTime) { return Promise.resolve(); };
    OfflineAudioContextProto.resume = function() { return Promise.resolve(); };

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
