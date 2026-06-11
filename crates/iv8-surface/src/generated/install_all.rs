//! Generated install_all — creates all templates in topological order.

use v8::Local;
use v8::Object;
use v8::FunctionTemplate;

/// Empty constructor (cannot be inlined — referenced by each template).
unsafe extern "C" fn empty_constructor(_info: *const v8::FunctionCallbackInfo) {}

pub fn install_all(scope: &v8::PinScope<'_, '_>, global: Local<Object>) {
    let mut templates: std::collections::HashMap<&str, v8::Local<FunctionTemplate>> = std::collections::HashMap::new();

    let tmpl_angle_instanced_arrays = super::web_apis::create_angle_instanced_arrays_template(scope, None);
    templates.insert("ANGLE_instanced_arrays", tmpl_angle_instanced_arrays);
    let tmpl_abort_controller = super::dom_core::create_abort_controller_template(scope, None);
    templates.insert("AbortController", tmpl_abort_controller);
    let tmpl_abstract_range = super::dom_core::create_abstract_range_template(scope, None);
    templates.insert("AbstractRange", tmpl_abstract_range);
    let tmpl_animation_effect = super::web_apis::create_animation_effect_template(scope, None);
    templates.insert("AnimationEffect", tmpl_animation_effect);
    let tmpl_animation_node_list = super::web_apis::create_animation_node_list_template(scope, None);
    templates.insert("AnimationNodeList", tmpl_animation_node_list);
    let tmpl_animation_timeline = super::web_apis::create_animation_timeline_template(scope, None);
    templates.insert("AnimationTimeline", tmpl_animation_timeline);
    let tmpl_animation_trigger = super::web_apis::create_animation_trigger_template(scope, None);
    templates.insert("AnimationTrigger", tmpl_animation_trigger);
    let tmpl_attribution = super::web_apis::create_attribution_template(scope, None);
    templates.insert("Attribution", tmpl_attribution);
    let tmpl_attribution_aggregation_services = super::web_apis::create_attribution_aggregation_services_template(scope, None);
    templates.insert("AttributionAggregationServices", tmpl_attribution_aggregation_services);
    let tmpl_audio_buffer = super::web_audio::create_audio_buffer_template(scope, None);
    templates.insert("AudioBuffer", tmpl_audio_buffer);
    let tmpl_audio_data = super::web_audio::create_audio_data_template(scope, None);
    templates.insert("AudioData", tmpl_audio_data);
    let tmpl_audio_listener = super::web_audio::create_audio_listener_template(scope, None);
    templates.insert("AudioListener", tmpl_audio_listener);
    let tmpl_audio_param = super::web_audio::create_audio_param_template(scope, None);
    templates.insert("AudioParam", tmpl_audio_param);
    let tmpl_audio_param_map = super::web_audio::create_audio_param_map_template(scope, None);
    templates.insert("AudioParamMap", tmpl_audio_param_map);
    let tmpl_audio_playback_stats = super::web_audio::create_audio_playback_stats_template(scope, None);
    templates.insert("AudioPlaybackStats", tmpl_audio_playback_stats);
    let tmpl_audio_sink_info = super::web_audio::create_audio_sink_info_template(scope, None);
    templates.insert("AudioSinkInfo", tmpl_audio_sink_info);
    let tmpl_audio_track = super::web_audio::create_audio_track_template(scope, None);
    templates.insert("AudioTrack", tmpl_audio_track);
    let tmpl_audio_worklet_processor = super::web_audio::create_audio_worklet_processor_template(scope, None);
    templates.insert("AudioWorkletProcessor", tmpl_audio_worklet_processor);
    let tmpl_authenticator_response = super::web_apis::create_authenticator_response_template(scope, None);
    templates.insert("AuthenticatorResponse", tmpl_authenticator_response);
    let tmpl_background_fetch_manager = super::web_apis::create_background_fetch_manager_template(scope, None);
    templates.insert("BackgroundFetchManager", tmpl_background_fetch_manager);
    let tmpl_background_fetch_record = super::web_apis::create_background_fetch_record_template(scope, None);
    templates.insert("BackgroundFetchRecord", tmpl_background_fetch_record);
    let tmpl_bar_prop = super::web_apis::create_bar_prop_template(scope, None);
    templates.insert("BarProp", tmpl_bar_prop);
    let tmpl_barcode_detector = super::web_apis::create_barcode_detector_template(scope, None);
    templates.insert("BarcodeDetector", tmpl_barcode_detector);
    let tmpl_baseline = super::web_apis::create_baseline_template(scope, None);
    templates.insert("Baseline", tmpl_baseline);
    let tmpl_blob = super::web_apis::create_blob_template(scope, None);
    templates.insert("Blob", tmpl_blob);
    let tmpl_bluetooth_characteristic_properties = super::bluetooth::create_bluetooth_characteristic_properties_template(scope, None);
    templates.insert("BluetoothCharacteristicProperties", tmpl_bluetooth_characteristic_properties);
    let tmpl_bluetooth_data_filter = super::bluetooth::create_bluetooth_data_filter_template(scope, None);
    templates.insert("BluetoothDataFilter", tmpl_bluetooth_data_filter);
    let tmpl_bluetooth_lescan = super::bluetooth::create_bluetooth_lescan_template(scope, None);
    templates.insert("BluetoothLEScan", tmpl_bluetooth_lescan);
    let tmpl_bluetooth_lescan_filter = super::bluetooth::create_bluetooth_lescan_filter_template(scope, None);
    templates.insert("BluetoothLEScanFilter", tmpl_bluetooth_lescan_filter);
    let tmpl_bluetooth_manufacturer_data_filter = super::bluetooth::create_bluetooth_manufacturer_data_filter_template(scope, None);
    templates.insert("BluetoothManufacturerDataFilter", tmpl_bluetooth_manufacturer_data_filter);
    let tmpl_bluetooth_manufacturer_data_map = super::bluetooth::create_bluetooth_manufacturer_data_map_template(scope, None);
    templates.insert("BluetoothManufacturerDataMap", tmpl_bluetooth_manufacturer_data_map);
    let tmpl_bluetooth_remote_gattdescriptor = super::bluetooth::create_bluetooth_remote_gattdescriptor_template(scope, None);
    templates.insert("BluetoothRemoteGATTDescriptor", tmpl_bluetooth_remote_gattdescriptor);
    let tmpl_bluetooth_remote_gattserver = super::bluetooth::create_bluetooth_remote_gattserver_template(scope, None);
    templates.insert("BluetoothRemoteGATTServer", tmpl_bluetooth_remote_gattserver);
    let tmpl_bluetooth_service_data_filter = super::bluetooth::create_bluetooth_service_data_filter_template(scope, None);
    templates.insert("BluetoothServiceDataFilter", tmpl_bluetooth_service_data_filter);
    let tmpl_bluetooth_service_data_map = super::bluetooth::create_bluetooth_service_data_map_template(scope, None);
    templates.insert("BluetoothServiceDataMap", tmpl_bluetooth_service_data_map);
    let tmpl_bluetooth_uuid = super::bluetooth::create_bluetooth_uuid_template(scope, None);
    templates.insert("BluetoothUUID", tmpl_bluetooth_uuid);
    let tmpl_break_token = super::web_apis::create_break_token_template(scope, None);
    templates.insert("BreakToken", tmpl_break_token);
    let tmpl_byte_length_queuing_strategy = super::streams::create_byte_length_queuing_strategy_template(scope, None);
    templates.insert("ByteLengthQueuingStrategy", tmpl_byte_length_queuing_strategy);
    let tmpl_cssfont_feature_values_map = super::css_om::create_cssfont_feature_values_map_template(scope, None);
    templates.insert("CSSFontFeatureValuesMap", tmpl_cssfont_feature_values_map);
    let tmpl_cssnumeric_array = super::css_om::create_cssnumeric_array_template(scope, None);
    templates.insert("CSSNumericArray", tmpl_cssnumeric_array);
    let tmpl_cssparser_rule = super::css_om::create_cssparser_rule_template(scope, None);
    templates.insert("CSSParserRule", tmpl_cssparser_rule);
    let tmpl_cssparser_value = super::css_om::create_cssparser_value_template(scope, None);
    templates.insert("CSSParserValue", tmpl_cssparser_value);
    let tmpl_csspseudo_element = super::css_om::create_csspseudo_element_template(scope, None);
    templates.insert("CSSPseudoElement", tmpl_csspseudo_element);
    let tmpl_cssrule = super::css_om::create_cssrule_template(scope, None);
    templates.insert("CSSRule", tmpl_cssrule);
    let tmpl_cssrule_list = super::css_om::create_cssrule_list_template(scope, None);
    templates.insert("CSSRuleList", tmpl_cssrule_list);
    let tmpl_cssstyle_declaration = super::css_om::create_cssstyle_declaration_template(scope, None);
    templates.insert("CSSStyleDeclaration", tmpl_cssstyle_declaration);
    let tmpl_cssstyle_value = super::css_om::create_cssstyle_value_template(scope, None);
    templates.insert("CSSStyleValue", tmpl_cssstyle_value);
    let tmpl_csstransform_component = super::css_om::create_csstransform_component_template(scope, None);
    templates.insert("CSSTransformComponent", tmpl_csstransform_component);
    let tmpl_cssvariable_reference_value = super::css_om::create_cssvariable_reference_value_template(scope, None);
    templates.insert("CSSVariableReferenceValue", tmpl_cssvariable_reference_value);
    let tmpl_cache = super::web_apis::create_cache_template(scope, None);
    templates.insert("Cache", tmpl_cache);
    let tmpl_cache_storage = super::web_apis::create_cache_storage_template(scope, None);
    templates.insert("CacheStorage", tmpl_cache_storage);
    let tmpl_canvas_gradient = super::web_apis::create_canvas_gradient_template(scope, None);
    templates.insert("CanvasGradient", tmpl_canvas_gradient);
    let tmpl_canvas_pattern = super::web_apis::create_canvas_pattern_template(scope, None);
    templates.insert("CanvasPattern", tmpl_canvas_pattern);
    let tmpl_canvas_rendering_context2_d = super::web_apis::create_canvas_rendering_context2_d_template(scope, None);
    templates.insert("CanvasRenderingContext2D", tmpl_canvas_rendering_context2_d);
    let tmpl_caret_position = super::web_apis::create_caret_position_template(scope, None);
    templates.insert("CaretPosition", tmpl_caret_position);
    let tmpl_chapter_information = super::web_apis::create_chapter_information_template(scope, None);
    templates.insert("ChapterInformation", tmpl_chapter_information);
    let tmpl_child_break_token = super::web_apis::create_child_break_token_template(scope, None);
    templates.insert("ChildBreakToken", tmpl_child_break_token);
    let tmpl_chrome_accessibility_features = super::chrome_extensions::create_chrome_accessibility_features_template(scope, None);
    templates.insert("ChromeAccessibilityFeatures", tmpl_chrome_accessibility_features);
    let tmpl_chrome_action = super::chrome_extensions::create_chrome_action_template(scope, None);
    templates.insert("ChromeAction", tmpl_chrome_action);
    let tmpl_chrome_alarms = super::chrome_extensions::create_chrome_alarms_template(scope, None);
    templates.insert("ChromeAlarms", tmpl_chrome_alarms);
    let tmpl_chrome_app = super::chrome_extensions::create_chrome_app_template(scope, None);
    templates.insert("ChromeApp", tmpl_chrome_app);
    let tmpl_chrome_app_runtime = super::chrome_extensions::create_chrome_app_runtime_template(scope, None);
    templates.insert("ChromeAppRuntime", tmpl_chrome_app_runtime);
    let tmpl_chrome_app_window = super::chrome_extensions::create_chrome_app_window_template(scope, None);
    templates.insert("ChromeAppWindow", tmpl_chrome_app_window);
    let tmpl_chrome_audio = super::chrome_extensions::create_chrome_audio_template(scope, None);
    templates.insert("ChromeAudio", tmpl_chrome_audio);
    let tmpl_chrome_autofill_private = super::chrome_extensions::create_chrome_autofill_private_template(scope, None);
    templates.insert("ChromeAutofillPrivate", tmpl_chrome_autofill_private);
    let tmpl_chrome_bluetooth = super::chrome_extensions::create_chrome_bluetooth_template(scope, None);
    templates.insert("ChromeBluetooth", tmpl_chrome_bluetooth);
    let tmpl_chrome_bluetooth_low_energy = super::chrome_extensions::create_chrome_bluetooth_low_energy_template(scope, None);
    templates.insert("ChromeBluetoothLowEnergy", tmpl_chrome_bluetooth_low_energy);
    let tmpl_chrome_bluetooth_socket = super::chrome_extensions::create_chrome_bluetooth_socket_template(scope, None);
    templates.insert("ChromeBluetoothSocket", tmpl_chrome_bluetooth_socket);
    let tmpl_chrome_bookmarks = super::chrome_extensions::create_chrome_bookmarks_template(scope, None);
    templates.insert("ChromeBookmarks", tmpl_chrome_bookmarks);
    let tmpl_chrome_braille_display_private = super::chrome_extensions::create_chrome_braille_display_private_template(scope, None);
    templates.insert("ChromeBrailleDisplayPrivate", tmpl_chrome_braille_display_private);
    let tmpl_chrome_browsing_data = super::chrome_extensions::create_chrome_browsing_data_template(scope, None);
    templates.insert("ChromeBrowsingData", tmpl_chrome_browsing_data);
    let tmpl_chrome_csi = super::chrome_extensions::create_chrome_csi_template(scope, None);
    templates.insert("ChromeCSI", tmpl_chrome_csi);
    let tmpl_chrome_cast = super::chrome_extensions::create_chrome_cast_template(scope, None);
    templates.insert("ChromeCast", tmpl_chrome_cast);
    let tmpl_chrome_cast_streaming = super::chrome_extensions::create_chrome_cast_streaming_template(scope, None);
    templates.insert("ChromeCastStreaming", tmpl_chrome_cast_streaming);
    let tmpl_chrome_certificate_provider = super::chrome_extensions::create_chrome_certificate_provider_template(scope, None);
    templates.insert("ChromeCertificateProvider", tmpl_chrome_certificate_provider);
    let tmpl_chrome_clipboard = super::chrome_extensions::create_chrome_clipboard_template(scope, None);
    templates.insert("ChromeClipboard", tmpl_chrome_clipboard);
    let tmpl_chrome_command_line_private = super::chrome_extensions::create_chrome_command_line_private_template(scope, None);
    templates.insert("ChromeCommandLinePrivate", tmpl_chrome_command_line_private);
    let tmpl_chrome_commands = super::chrome_extensions::create_chrome_commands_template(scope, None);
    templates.insert("ChromeCommands", tmpl_chrome_commands);
    let tmpl_chrome_content_settings = super::chrome_extensions::create_chrome_content_settings_template(scope, None);
    templates.insert("ChromeContentSettings", tmpl_chrome_content_settings);
    let tmpl_chrome_context_menus = super::chrome_extensions::create_chrome_context_menus_template(scope, None);
    templates.insert("ChromeContextMenus", tmpl_chrome_context_menus);
    let tmpl_chrome_cookies = super::chrome_extensions::create_chrome_cookies_template(scope, None);
    templates.insert("ChromeCookies", tmpl_chrome_cookies);
    let tmpl_chrome_dom = super::chrome_extensions::create_chrome_dom_template(scope, None);
    templates.insert("ChromeDOM", tmpl_chrome_dom);
    let tmpl_chrome_debugger = super::chrome_extensions::create_chrome_debugger_template(scope, None);
    templates.insert("ChromeDebugger", tmpl_chrome_debugger);
    let tmpl_chrome_declarative_content = super::chrome_extensions::create_chrome_declarative_content_template(scope, None);
    templates.insert("ChromeDeclarativeContent", tmpl_chrome_declarative_content);
    let tmpl_chrome_declarative_net_request = super::chrome_extensions::create_chrome_declarative_net_request_template(scope, None);
    templates.insert("ChromeDeclarativeNetRequest", tmpl_chrome_declarative_net_request);
    let tmpl_chrome_desk_capture = super::chrome_extensions::create_chrome_desk_capture_template(scope, None);
    templates.insert("ChromeDeskCapture", tmpl_chrome_desk_capture);
    let tmpl_chrome_desktop_capture = super::chrome_extensions::create_chrome_desktop_capture_template(scope, None);
    templates.insert("ChromeDesktopCapture", tmpl_chrome_desktop_capture);
    let tmpl_chrome_devtools_namespace = super::chrome_extensions::create_chrome_devtools_namespace_template(scope, None);
    templates.insert("ChromeDevtoolsNamespace", tmpl_chrome_devtools_namespace);
    let tmpl_chrome_diagnostics = super::chrome_extensions::create_chrome_diagnostics_template(scope, None);
    templates.insert("ChromeDiagnostics", tmpl_chrome_diagnostics);
    let tmpl_chrome_display_source = super::chrome_extensions::create_chrome_display_source_template(scope, None);
    templates.insert("ChromeDisplaySource", tmpl_chrome_display_source);
    let tmpl_chrome_dns = super::chrome_extensions::create_chrome_dns_template(scope, None);
    templates.insert("ChromeDns", tmpl_chrome_dns);
    let tmpl_chrome_document_scan = super::chrome_extensions::create_chrome_document_scan_template(scope, None);
    templates.insert("ChromeDocumentScan", tmpl_chrome_document_scan);
    let tmpl_chrome_downloads = super::chrome_extensions::create_chrome_downloads_template(scope, None);
    templates.insert("ChromeDownloads", tmpl_chrome_downloads);
    let tmpl_chrome_echo_private = super::chrome_extensions::create_chrome_echo_private_template(scope, None);
    templates.insert("ChromeEchoPrivate", tmpl_chrome_echo_private);
    let tmpl_chrome_enterprise_hardware_platform = super::chrome_extensions::create_chrome_enterprise_hardware_platform_template(scope, None);
    templates.insert("ChromeEnterpriseHardwarePlatform", tmpl_chrome_enterprise_hardware_platform);
    let tmpl_chrome_enterprise_ns = super::chrome_extensions::create_chrome_enterprise_ns_template(scope, None);
    templates.insert("ChromeEnterpriseNS", tmpl_chrome_enterprise_ns);
    let tmpl_chrome_enterprise_platform_keys = super::chrome_extensions::create_chrome_enterprise_platform_keys_template(scope, None);
    templates.insert("ChromeEnterprisePlatformKeys", tmpl_chrome_enterprise_platform_keys);
    let tmpl_chrome_event = super::events::create_chrome_event_template(scope, None);
    templates.insert("ChromeEvent", tmpl_chrome_event);
    let tmpl_chrome_events = super::chrome_extensions::create_chrome_events_template(scope, None);
    templates.insert("ChromeEvents", tmpl_chrome_events);
    let tmpl_chrome_experience_sampling_private = super::chrome_extensions::create_chrome_experience_sampling_private_template(scope, None);
    templates.insert("ChromeExperienceSamplingPrivate", tmpl_chrome_experience_sampling_private);
    let tmpl_chrome_extension = super::chrome_extensions::create_chrome_extension_template(scope, None);
    templates.insert("ChromeExtension", tmpl_chrome_extension);
    let tmpl_chrome_feedback_private = super::chrome_extensions::create_chrome_feedback_private_template(scope, None);
    templates.insert("ChromeFeedbackPrivate", tmpl_chrome_feedback_private);
    let tmpl_chrome_file_browser_handler = super::chrome_extensions::create_chrome_file_browser_handler_template(scope, None);
    templates.insert("ChromeFileBrowserHandler", tmpl_chrome_file_browser_handler);
    let tmpl_chrome_file_system_provider = super::chrome_extensions::create_chrome_file_system_provider_template(scope, None);
    templates.insert("ChromeFileSystemProvider", tmpl_chrome_file_system_provider);
    let tmpl_chrome_first_run_private = super::chrome_extensions::create_chrome_first_run_private_template(scope, None);
    templates.insert("ChromeFirstRunPrivate", tmpl_chrome_first_run_private);
    let tmpl_chrome_font_settings = super::chrome_extensions::create_chrome_font_settings_template(scope, None);
    templates.insert("ChromeFontSettings", tmpl_chrome_font_settings);
    let tmpl_chrome_gcm = super::chrome_extensions::create_chrome_gcm_template(scope, None);
    templates.insert("ChromeGcm", tmpl_chrome_gcm);
    let tmpl_chrome_hid = super::chrome_extensions::create_chrome_hid_template(scope, None);
    templates.insert("ChromeHid", tmpl_chrome_hid);
    let tmpl_chrome_history = super::chrome_extensions::create_chrome_history_template(scope, None);
    templates.insert("ChromeHistory", tmpl_chrome_history);
    let tmpl_chrome_hotword_private = super::chrome_extensions::create_chrome_hotword_private_template(scope, None);
    templates.insert("ChromeHotwordPrivate", tmpl_chrome_hotword_private);
    let tmpl_chrome_i18n = super::chrome_extensions::create_chrome_i18n_template(scope, None);
    templates.insert("ChromeI18n", tmpl_chrome_i18n);
    let tmpl_chrome_identity = super::chrome_extensions::create_chrome_identity_template(scope, None);
    templates.insert("ChromeIdentity", tmpl_chrome_identity);
    let tmpl_chrome_identity_private = super::chrome_extensions::create_chrome_identity_private_template(scope, None);
    templates.insert("ChromeIdentityPrivate", tmpl_chrome_identity_private);
    let tmpl_chrome_identity_private_api = super::chrome_extensions::create_chrome_identity_private_api_template(scope, None);
    templates.insert("ChromeIdentityPrivateAPI", tmpl_chrome_identity_private_api);
    let tmpl_chrome_idle = super::chrome_extensions::create_chrome_idle_template(scope, None);
    templates.insert("ChromeIdle", tmpl_chrome_idle);
    let tmpl_chrome_idle_private = super::chrome_extensions::create_chrome_idle_private_template(scope, None);
    templates.insert("ChromeIdlePrivate", tmpl_chrome_idle_private);
    let tmpl_chrome_input_ime = super::chrome_extensions::create_chrome_input_ime_template(scope, None);
    templates.insert("ChromeInputIme", tmpl_chrome_input_ime);
    let tmpl_chrome_input_method_private = super::chrome_extensions::create_chrome_input_method_private_template(scope, None);
    templates.insert("ChromeInputMethodPrivate", tmpl_chrome_input_method_private);
    let tmpl_chrome_instance_id = super::chrome_extensions::create_chrome_instance_id_template(scope, None);
    templates.insert("ChromeInstanceID", tmpl_chrome_instance_id);
    let tmpl_chrome_language_settings_private = super::chrome_extensions::create_chrome_language_settings_private_template(scope, None);
    templates.insert("ChromeLanguageSettingsPrivate", tmpl_chrome_language_settings_private);
    let tmpl_chrome_load_times = super::chrome_extensions::create_chrome_load_times_template(scope, None);
    templates.insert("ChromeLoadTimes", tmpl_chrome_load_times);
    let tmpl_chrome_location = super::chrome_extensions::create_chrome_location_template(scope, None);
    templates.insert("ChromeLocation", tmpl_chrome_location);
    let tmpl_chrome_log_private = super::chrome_extensions::create_chrome_log_private_template(scope, None);
    templates.insert("ChromeLogPrivate", tmpl_chrome_log_private);
    let tmpl_chrome_login_state = super::chrome_extensions::create_chrome_login_state_template(scope, None);
    templates.insert("ChromeLoginState", tmpl_chrome_login_state);
    let tmpl_chrome_management = super::chrome_extensions::create_chrome_management_template(scope, None);
    templates.insert("ChromeManagement", tmpl_chrome_management);
    let tmpl_chrome_mdns = super::chrome_extensions::create_chrome_mdns_template(scope, None);
    templates.insert("ChromeMdns", tmpl_chrome_mdns);
    let tmpl_chrome_media_galleries = super::chrome_extensions::create_chrome_media_galleries_template(scope, None);
    templates.insert("ChromeMediaGalleries", tmpl_chrome_media_galleries);
    let tmpl_chrome_media_perception_private = super::chrome_extensions::create_chrome_media_perception_private_template(scope, None);
    templates.insert("ChromeMediaPerceptionPrivate", tmpl_chrome_media_perception_private);
    let tmpl_chrome_midi = super::chrome_extensions::create_chrome_midi_template(scope, None);
    templates.insert("ChromeMidi", tmpl_chrome_midi);
    let tmpl_chrome_music_manager_private = super::chrome_extensions::create_chrome_music_manager_private_template(scope, None);
    templates.insert("ChromeMusicManagerPrivate", tmpl_chrome_music_manager_private);
    let tmpl_chrome_networking_config = super::chrome_extensions::create_chrome_networking_config_template(scope, None);
    templates.insert("ChromeNetworkingConfig", tmpl_chrome_networking_config);
    let tmpl_chrome_networking_onc = super::chrome_extensions::create_chrome_networking_onc_template(scope, None);
    templates.insert("ChromeNetworkingOnc", tmpl_chrome_networking_onc);
    let tmpl_chrome_networking_private = super::chrome_extensions::create_chrome_networking_private_template(scope, None);
    templates.insert("ChromeNetworkingPrivate", tmpl_chrome_networking_private);
    let tmpl_chrome_notifications = super::chrome_extensions::create_chrome_notifications_template(scope, None);
    templates.insert("ChromeNotifications", tmpl_chrome_notifications);
    let tmpl_chrome_offscreen = super::chrome_extensions::create_chrome_offscreen_template(scope, None);
    templates.insert("ChromeOffscreen", tmpl_chrome_offscreen);
    let tmpl_chrome_omnibox = super::chrome_extensions::create_chrome_omnibox_template(scope, None);
    templates.insert("ChromeOmnibox", tmpl_chrome_omnibox);
    let tmpl_chrome_page_capture = super::chrome_extensions::create_chrome_page_capture_template(scope, None);
    templates.insert("ChromePageCapture", tmpl_chrome_page_capture);
    let tmpl_chrome_passwords_private = super::chrome_extensions::create_chrome_passwords_private_template(scope, None);
    templates.insert("ChromePasswordsPrivate", tmpl_chrome_passwords_private);
    let tmpl_chrome_permissions = super::chrome_extensions::create_chrome_permissions_template(scope, None);
    templates.insert("ChromePermissions", tmpl_chrome_permissions);
    let tmpl_chrome_platform_keys = super::chrome_extensions::create_chrome_platform_keys_template(scope, None);
    templates.insert("ChromePlatformKeys", tmpl_chrome_platform_keys);
    let tmpl_chrome_port = super::chrome_extensions::create_chrome_port_template(scope, None);
    templates.insert("ChromePort", tmpl_chrome_port);
    let tmpl_chrome_power = super::chrome_extensions::create_chrome_power_template(scope, None);
    templates.insert("ChromePower", tmpl_chrome_power);
    let tmpl_chrome_printer_provider = super::chrome_extensions::create_chrome_printer_provider_template(scope, None);
    templates.insert("ChromePrinterProvider", tmpl_chrome_printer_provider);
    let tmpl_chrome_printing_api = super::chrome_extensions::create_chrome_printing_api_template(scope, None);
    templates.insert("ChromePrintingAPI", tmpl_chrome_printing_api);
    let tmpl_chrome_printing_metrics = super::chrome_extensions::create_chrome_printing_metrics_template(scope, None);
    templates.insert("ChromePrintingMetrics", tmpl_chrome_printing_metrics);
    let tmpl_chrome_privacy = super::chrome_extensions::create_chrome_privacy_template(scope, None);
    templates.insert("ChromePrivacy", tmpl_chrome_privacy);
    let tmpl_chrome_processes = super::chrome_extensions::create_chrome_processes_template(scope, None);
    templates.insert("ChromeProcesses", tmpl_chrome_processes);
    let tmpl_chrome_proxy = super::chrome_extensions::create_chrome_proxy_template(scope, None);
    templates.insert("ChromeProxy", tmpl_chrome_proxy);
    let tmpl_chrome_quick_unlock_private = super::chrome_extensions::create_chrome_quick_unlock_private_template(scope, None);
    templates.insert("ChromeQuickUnlockPrivate", tmpl_chrome_quick_unlock_private);
    let tmpl_chrome_reading_list = super::chrome_extensions::create_chrome_reading_list_template(scope, None);
    templates.insert("ChromeReadingList", tmpl_chrome_reading_list);
    let tmpl_chrome_resource_private = super::chrome_extensions::create_chrome_resource_private_template(scope, None);
    templates.insert("ChromeResourcePrivate", tmpl_chrome_resource_private);
    let tmpl_chrome_runtime = super::chrome_extensions::create_chrome_runtime_template(scope, None);
    templates.insert("ChromeRuntime", tmpl_chrome_runtime);
    let tmpl_chrome_runtime_on_installed = super::chrome_extensions::create_chrome_runtime_on_installed_template(scope, None);
    templates.insert("ChromeRuntimeOnInstalled", tmpl_chrome_runtime_on_installed);
    let tmpl_chrome_runtime_private = super::chrome_extensions::create_chrome_runtime_private_template(scope, None);
    templates.insert("ChromeRuntimePrivate", tmpl_chrome_runtime_private);
    let tmpl_chrome_safe_browsing_private = super::chrome_extensions::create_chrome_safe_browsing_private_template(scope, None);
    templates.insert("ChromeSafeBrowsingPrivate", tmpl_chrome_safe_browsing_private);
    let tmpl_chrome_scripting = super::chrome_extensions::create_chrome_scripting_template(scope, None);
    templates.insert("ChromeScripting", tmpl_chrome_scripting);
    let tmpl_chrome_search = super::chrome_extensions::create_chrome_search_template(scope, None);
    templates.insert("ChromeSearch", tmpl_chrome_search);
    let tmpl_chrome_serial = super::chrome_extensions::create_chrome_serial_template(scope, None);
    templates.insert("ChromeSerial", tmpl_chrome_serial);
    let tmpl_chrome_sessions = super::chrome_extensions::create_chrome_sessions_template(scope, None);
    templates.insert("ChromeSessions", tmpl_chrome_sessions);
    let tmpl_chrome_settings_private = super::chrome_extensions::create_chrome_settings_private_template(scope, None);
    templates.insert("ChromeSettingsPrivate", tmpl_chrome_settings_private);
    let tmpl_chrome_side_panel = super::chrome_extensions::create_chrome_side_panel_template(scope, None);
    templates.insert("ChromeSidePanel", tmpl_chrome_side_panel);
    let tmpl_chrome_signed_in_devices = super::chrome_extensions::create_chrome_signed_in_devices_template(scope, None);
    templates.insert("ChromeSignedInDevices", tmpl_chrome_signed_in_devices);
    let tmpl_chrome_socket = super::chrome_extensions::create_chrome_socket_template(scope, None);
    templates.insert("ChromeSocket", tmpl_chrome_socket);
    let tmpl_chrome_speech_recognition_private = super::chrome_extensions::create_chrome_speech_recognition_private_template(scope, None);
    templates.insert("ChromeSpeechRecognitionPrivate", tmpl_chrome_speech_recognition_private);
    let tmpl_chrome_storage = super::chrome_extensions::create_chrome_storage_template(scope, None);
    templates.insert("ChromeStorage", tmpl_chrome_storage);
    let tmpl_chrome_storage_area = super::chrome_extensions::create_chrome_storage_area_template(scope, None);
    templates.insert("ChromeStorageArea", tmpl_chrome_storage_area);
    let tmpl_chrome_storage_managed = super::chrome_extensions::create_chrome_storage_managed_template(scope, None);
    templates.insert("ChromeStorageManaged", tmpl_chrome_storage_managed);
    let tmpl_chrome_sync_file_system = super::chrome_extensions::create_chrome_sync_file_system_template(scope, None);
    templates.insert("ChromeSyncFileSystem", tmpl_chrome_sync_file_system);
    let tmpl_chrome_system_cpu = super::chrome_extensions::create_chrome_system_cpu_template(scope, None);
    templates.insert("ChromeSystemCpu", tmpl_chrome_system_cpu);
    let tmpl_chrome_system_display = super::chrome_extensions::create_chrome_system_display_template(scope, None);
    templates.insert("ChromeSystemDisplay", tmpl_chrome_system_display);
    let tmpl_chrome_system_memory = super::chrome_extensions::create_chrome_system_memory_template(scope, None);
    templates.insert("ChromeSystemMemory", tmpl_chrome_system_memory);
    let tmpl_chrome_system_ns = super::chrome_extensions::create_chrome_system_ns_template(scope, None);
    templates.insert("ChromeSystemNS", tmpl_chrome_system_ns);
    let tmpl_chrome_system_network = super::chrome_extensions::create_chrome_system_network_template(scope, None);
    templates.insert("ChromeSystemNetwork", tmpl_chrome_system_network);
    let tmpl_chrome_system_storage = super::chrome_extensions::create_chrome_system_storage_template(scope, None);
    templates.insert("ChromeSystemStorage", tmpl_chrome_system_storage);
    let tmpl_chrome_tab_capture = super::chrome_extensions::create_chrome_tab_capture_template(scope, None);
    templates.insert("ChromeTabCapture", tmpl_chrome_tab_capture);
    let tmpl_chrome_tab_groups = super::chrome_extensions::create_chrome_tab_groups_template(scope, None);
    templates.insert("ChromeTabGroups", tmpl_chrome_tab_groups);
    let tmpl_chrome_tabs = super::chrome_extensions::create_chrome_tabs_template(scope, None);
    templates.insert("ChromeTabs", tmpl_chrome_tabs);
    let tmpl_chrome_top_sites = super::chrome_extensions::create_chrome_top_sites_template(scope, None);
    templates.insert("ChromeTopSites", tmpl_chrome_top_sites);
    let tmpl_chrome_tts = super::chrome_extensions::create_chrome_tts_template(scope, None);
    templates.insert("ChromeTts", tmpl_chrome_tts);
    let tmpl_chrome_tts_engine = super::chrome_extensions::create_chrome_tts_engine_template(scope, None);
    templates.insert("ChromeTtsEngine", tmpl_chrome_tts_engine);
    let tmpl_chrome_types = super::chrome_extensions::create_chrome_types_template(scope, None);
    templates.insert("ChromeTypes", tmpl_chrome_types);
    let tmpl_chrome_usb = super::chrome_extensions::create_chrome_usb_template(scope, None);
    templates.insert("ChromeUsb", tmpl_chrome_usb);
    let tmpl_chrome_user_scripts = super::chrome_extensions::create_chrome_user_scripts_template(scope, None);
    templates.insert("ChromeUserScripts", tmpl_chrome_user_scripts);
    let tmpl_chrome_virtual_keyboard_private = super::chrome_extensions::create_chrome_virtual_keyboard_private_template(scope, None);
    templates.insert("ChromeVirtualKeyboardPrivate", tmpl_chrome_virtual_keyboard_private);
    let tmpl_chrome_vpn_provider = super::chrome_extensions::create_chrome_vpn_provider_template(scope, None);
    templates.insert("ChromeVpnProvider", tmpl_chrome_vpn_provider);
    let tmpl_chrome_wallpaper = super::chrome_extensions::create_chrome_wallpaper_template(scope, None);
    templates.insert("ChromeWallpaper", tmpl_chrome_wallpaper);
    let tmpl_chrome_web_authentication_proxy = super::chrome_extensions::create_chrome_web_authentication_proxy_template(scope, None);
    templates.insert("ChromeWebAuthenticationProxy", tmpl_chrome_web_authentication_proxy);
    let tmpl_chrome_web_navigation = super::chrome_extensions::create_chrome_web_navigation_template(scope, None);
    templates.insert("ChromeWebNavigation", tmpl_chrome_web_navigation);
    let tmpl_chrome_web_request = super::chrome_extensions::create_chrome_web_request_template(scope, None);
    templates.insert("ChromeWebRequest", tmpl_chrome_web_request);
    let tmpl_chrome_web_view = super::chrome_extensions::create_chrome_web_view_template(scope, None);
    templates.insert("ChromeWebView", tmpl_chrome_web_view);
    let tmpl_chrome_webrtc_audio_private = super::chrome_extensions::create_chrome_webrtc_audio_private_template(scope, None);
    templates.insert("ChromeWebrtcAudioPrivate", tmpl_chrome_webrtc_audio_private);
    let tmpl_chrome_webrtc_desktop_capture_private = super::chrome_extensions::create_chrome_webrtc_desktop_capture_private_template(scope, None);
    templates.insert("ChromeWebrtcDesktopCapturePrivate", tmpl_chrome_webrtc_desktop_capture_private);
    let tmpl_chrome_webrtc_logging_private = super::chrome_extensions::create_chrome_webrtc_logging_private_template(scope, None);
    templates.insert("ChromeWebrtcLoggingPrivate", tmpl_chrome_webrtc_logging_private);
    let tmpl_chrome_webstore = super::chrome_extensions::create_chrome_webstore_template(scope, None);
    templates.insert("ChromeWebstore", tmpl_chrome_webstore);
    let tmpl_chrome_webstore_private = super::chrome_extensions::create_chrome_webstore_private_template(scope, None);
    templates.insert("ChromeWebstorePrivate", tmpl_chrome_webstore_private);
    let tmpl_chrome_windows = super::chrome_extensions::create_chrome_windows_template(scope, None);
    templates.insert("ChromeWindows", tmpl_chrome_windows);
    let tmpl_client = super::web_apis::create_client_template(scope, None);
    templates.insert("Client", tmpl_client);
    let tmpl_clients = super::web_apis::create_clients_template(scope, None);
    templates.insert("Clients", tmpl_clients);
    let tmpl_clipboard_item = super::web_apis::create_clipboard_item_template(scope, None);
    templates.insert("ClipboardItem", tmpl_clipboard_item);
    let tmpl_compression_stream = super::web_apis::create_compression_stream_template(scope, None);
    templates.insert("CompressionStream", tmpl_compression_stream);
    let tmpl_contact_address = super::web_apis::create_contact_address_template(scope, None);
    templates.insert("ContactAddress", tmpl_contact_address);
    let tmpl_contacts_manager = super::web_apis::create_contacts_manager_template(scope, None);
    templates.insert("ContactsManager", tmpl_contacts_manager);
    let tmpl_content_index = super::web_apis::create_content_index_template(scope, None);
    templates.insert("ContentIndex", tmpl_content_index);
    let tmpl_cookie_store_manager = super::web_apis::create_cookie_store_manager_template(scope, None);
    templates.insert("CookieStoreManager", tmpl_cookie_store_manager);
    let tmpl_count_queuing_strategy = super::streams::create_count_queuing_strategy_template(scope, None);
    templates.insert("CountQueuingStrategy", tmpl_count_queuing_strategy);
    let tmpl_crash_report_context = super::web_apis::create_crash_report_context_template(scope, None);
    templates.insert("CrashReportContext", tmpl_crash_report_context);
    let tmpl_credential = super::web_apis::create_credential_template(scope, None);
    templates.insert("Credential", tmpl_credential);
    let tmpl_credentials_container = super::web_apis::create_credentials_container_template(scope, None);
    templates.insert("CredentialsContainer", tmpl_credentials_container);
    let tmpl_crop_target = super::web_apis::create_crop_target_template(scope, None);
    templates.insert("CropTarget", tmpl_crop_target);
    let tmpl_crypto = super::crypto::create_crypto_template(scope, None);
    templates.insert("Crypto", tmpl_crypto);
    let tmpl_crypto_key = super::crypto::create_crypto_key_template(scope, None);
    templates.insert("CryptoKey", tmpl_crypto_key);
    let tmpl_custom_element_registry = super::web_apis::create_custom_element_registry_template(scope, None);
    templates.insert("CustomElementRegistry", tmpl_custom_element_registry);
    let tmpl_custom_state_set = super::web_apis::create_custom_state_set_template(scope, None);
    templates.insert("CustomStateSet", tmpl_custom_state_set);
    let tmpl_domexception = super::web_apis::create_domexception_template(scope, None);
    templates.insert("DOMException", tmpl_domexception);
    let tmpl_domimplementation = super::dom_core::create_domimplementation_template(scope, None);
    templates.insert("DOMImplementation", tmpl_domimplementation);
    let tmpl_dommatrix_read_only = super::dom_core::create_dommatrix_read_only_template(scope, None);
    templates.insert("DOMMatrixReadOnly", tmpl_dommatrix_read_only);
    let tmpl_domparser = super::web_apis::create_domparser_template(scope, None);
    templates.insert("DOMParser", tmpl_domparser);
    let tmpl_dompoint_read_only = super::dom_core::create_dompoint_read_only_template(scope, None);
    templates.insert("DOMPointReadOnly", tmpl_dompoint_read_only);
    let tmpl_domquad = super::dom_core::create_domquad_template(scope, None);
    templates.insert("DOMQuad", tmpl_domquad);
    let tmpl_domrect_list = super::web_apis::create_domrect_list_template(scope, None);
    templates.insert("DOMRectList", tmpl_domrect_list);
    let tmpl_domrect_read_only = super::dom_core::create_domrect_read_only_template(scope, None);
    templates.insert("DOMRectReadOnly", tmpl_domrect_read_only);
    let tmpl_domstring_list = super::web_apis::create_domstring_list_template(scope, None);
    templates.insert("DOMStringList", tmpl_domstring_list);
    let tmpl_domstring_map = super::web_apis::create_domstring_map_template(scope, None);
    templates.insert("DOMStringMap", tmpl_domstring_map);
    let tmpl_domtoken_list = super::dom_core::create_domtoken_list_template(scope, None);
    templates.insert("DOMTokenList", tmpl_domtoken_list);
    let tmpl_data_transfer = super::web_apis::create_data_transfer_template(scope, None);
    templates.insert("DataTransfer", tmpl_data_transfer);
    let tmpl_data_transfer_item = super::web_apis::create_data_transfer_item_template(scope, None);
    templates.insert("DataTransferItem", tmpl_data_transfer_item);
    let tmpl_data_transfer_item_list = super::web_apis::create_data_transfer_item_list_template(scope, None);
    templates.insert("DataTransferItemList", tmpl_data_transfer_item_list);
    let tmpl_decompression_stream = super::web_apis::create_decompression_stream_template(scope, None);
    templates.insert("DecompressionStream", tmpl_decompression_stream);
    let tmpl_delegated_ink_trail_presenter = super::web_apis::create_delegated_ink_trail_presenter_template(scope, None);
    templates.insert("DelegatedInkTrailPresenter", tmpl_delegated_ink_trail_presenter);
    let tmpl_device_motion_event_acceleration = super::web_apis::create_device_motion_event_acceleration_template(scope, None);
    templates.insert("DeviceMotionEventAcceleration", tmpl_device_motion_event_acceleration);
    let tmpl_device_motion_event_rotation_rate = super::web_apis::create_device_motion_event_rotation_rate_template(scope, None);
    templates.insert("DeviceMotionEventRotationRate", tmpl_device_motion_event_rotation_rate);
    let tmpl_digital_goods_service = super::web_apis::create_digital_goods_service_template(scope, None);
    templates.insert("DigitalGoodsService", tmpl_digital_goods_service);
    let tmpl_ext_blend_minmax = super::web_apis::create_ext_blend_minmax_template(scope, None);
    templates.insert("EXT_blend_minmax", tmpl_ext_blend_minmax);
    let tmpl_ext_color_buffer_float = super::web_apis::create_ext_color_buffer_float_template(scope, None);
    templates.insert("EXT_color_buffer_float", tmpl_ext_color_buffer_float);
    let tmpl_ext_color_buffer_half_float = super::web_apis::create_ext_color_buffer_half_float_template(scope, None);
    templates.insert("EXT_color_buffer_half_float", tmpl_ext_color_buffer_half_float);
    let tmpl_ext_disjoint_timer_query = super::web_apis::create_ext_disjoint_timer_query_template(scope, None);
    templates.insert("EXT_disjoint_timer_query", tmpl_ext_disjoint_timer_query);
    let tmpl_ext_disjoint_timer_query_webgl2 = super::web_apis::create_ext_disjoint_timer_query_webgl2_template(scope, None);
    templates.insert("EXT_disjoint_timer_query_webgl2", tmpl_ext_disjoint_timer_query_webgl2);
    let tmpl_ext_float_blend = super::web_apis::create_ext_float_blend_template(scope, None);
    templates.insert("EXT_float_blend", tmpl_ext_float_blend);
    let tmpl_ext_frag_depth = super::web_apis::create_ext_frag_depth_template(scope, None);
    templates.insert("EXT_frag_depth", tmpl_ext_frag_depth);
    let tmpl_ext_s_rgb = super::web_apis::create_ext_s_rgb_template(scope, None);
    templates.insert("EXT_sRGB", tmpl_ext_s_rgb);
    let tmpl_ext_shader_texture_lod = super::web_apis::create_ext_shader_texture_lod_template(scope, None);
    templates.insert("EXT_shader_texture_lod", tmpl_ext_shader_texture_lod);
    let tmpl_ext_texture_compression_bptc = super::web_apis::create_ext_texture_compression_bptc_template(scope, None);
    templates.insert("EXT_texture_compression_bptc", tmpl_ext_texture_compression_bptc);
    let tmpl_ext_texture_compression_rgtc = super::web_apis::create_ext_texture_compression_rgtc_template(scope, None);
    templates.insert("EXT_texture_compression_rgtc", tmpl_ext_texture_compression_rgtc);
    let tmpl_ext_texture_filter_anisotropic = super::web_apis::create_ext_texture_filter_anisotropic_template(scope, None);
    templates.insert("EXT_texture_filter_anisotropic", tmpl_ext_texture_filter_anisotropic);
    let tmpl_ext_texture_norm16 = super::web_apis::create_ext_texture_norm16_template(scope, None);
    templates.insert("EXT_texture_norm16", tmpl_ext_texture_norm16);
    let tmpl_element_internals = super::web_apis::create_element_internals_template(scope, None);
    templates.insert("ElementInternals", tmpl_element_internals);
    let tmpl_encoded_audio_chunk = super::web_apis::create_encoded_audio_chunk_template(scope, None);
    templates.insert("EncodedAudioChunk", tmpl_encoded_audio_chunk);
    let tmpl_encoded_video_chunk = super::web_apis::create_encoded_video_chunk_template(scope, None);
    templates.insert("EncodedVideoChunk", tmpl_encoded_video_chunk);
    let tmpl_event = super::events::create_event_template(scope, None);
    templates.insert("Event", tmpl_event);
    let tmpl_event_counts = super::events::create_event_counts_template(scope, None);
    templates.insert("EventCounts", tmpl_event_counts);
    let tmpl_event_target = super::dom_core::create_event_target_template(scope, None);
    templates.insert("EventTarget", tmpl_event_target);
    let tmpl_exception = super::web_apis::create_exception_template(scope, None);
    templates.insert("Exception", tmpl_exception);
    let tmpl_external = super::chrome_extensions::create_external_template(scope, None);
    templates.insert("External", tmpl_external);
    let tmpl_eye_dropper = super::web_apis::create_eye_dropper_template(scope, None);
    templates.insert("EyeDropper", tmpl_eye_dropper);
    let tmpl_face_detector = super::web_apis::create_face_detector_template(scope, None);
    templates.insert("FaceDetector", tmpl_face_detector);
    let tmpl_fence = super::web_apis::create_fence_template(scope, None);
    templates.insert("Fence", tmpl_fence);
    let tmpl_fenced_frame_config = super::web_apis::create_fenced_frame_config_template(scope, None);
    templates.insert("FencedFrameConfig", tmpl_fenced_frame_config);
    let tmpl_fetch_later_result = super::web_apis::create_fetch_later_result_template(scope, None);
    templates.insert("FetchLaterResult", tmpl_fetch_later_result);
    let tmpl_file_list = super::web_apis::create_file_list_template(scope, None);
    templates.insert("FileList", tmpl_file_list);
    let tmpl_file_reader_sync = super::web_apis::create_file_reader_sync_template(scope, None);
    templates.insert("FileReaderSync", tmpl_file_reader_sync);
    let tmpl_file_system = super::web_apis::create_file_system_template(scope, None);
    templates.insert("FileSystem", tmpl_file_system);
    let tmpl_file_system_directory_reader = super::web_apis::create_file_system_directory_reader_template(scope, None);
    templates.insert("FileSystemDirectoryReader", tmpl_file_system_directory_reader);
    let tmpl_file_system_entry = super::web_apis::create_file_system_entry_template(scope, None);
    templates.insert("FileSystemEntry", tmpl_file_system_entry);
    let tmpl_file_system_handle = super::web_apis::create_file_system_handle_template(scope, None);
    templates.insert("FileSystemHandle", tmpl_file_system_handle);
    let tmpl_file_system_sync_access_handle = super::web_apis::create_file_system_sync_access_handle_template(scope, None);
    templates.insert("FileSystemSyncAccessHandle", tmpl_file_system_sync_access_handle);
    let tmpl_font = super::web_apis::create_font_template(scope, None);
    templates.insert("Font", tmpl_font);
    let tmpl_font_data = super::web_apis::create_font_data_template(scope, None);
    templates.insert("FontData", tmpl_font_data);
    let tmpl_font_face = super::web_apis::create_font_face_template(scope, None);
    templates.insert("FontFace", tmpl_font_face);
    let tmpl_font_face_features = super::web_apis::create_font_face_features_template(scope, None);
    templates.insert("FontFaceFeatures", tmpl_font_face_features);
    let tmpl_font_face_palette = super::web_apis::create_font_face_palette_template(scope, None);
    templates.insert("FontFacePalette", tmpl_font_face_palette);
    let tmpl_font_face_palettes = super::web_apis::create_font_face_palettes_template(scope, None);
    templates.insert("FontFacePalettes", tmpl_font_face_palettes);
    let tmpl_font_face_variation_axis = super::web_apis::create_font_face_variation_axis_template(scope, None);
    templates.insert("FontFaceVariationAxis", tmpl_font_face_variation_axis);
    let tmpl_font_face_variations = super::web_apis::create_font_face_variations_template(scope, None);
    templates.insert("FontFaceVariations", tmpl_font_face_variations);
    let tmpl_font_metrics = super::web_apis::create_font_metrics_template(scope, None);
    templates.insert("FontMetrics", tmpl_font_metrics);
    let tmpl_form_data = super::fetch::create_form_data_template(scope, None);
    templates.insert("FormData", tmpl_form_data);
    let tmpl_fragment_directive = super::web_apis::create_fragment_directive_template(scope, None);
    templates.insert("FragmentDirective", tmpl_fragment_directive);
    let tmpl_fragment_result = super::web_apis::create_fragment_result_template(scope, None);
    templates.insert("FragmentResult", tmpl_fragment_result);
    let tmpl_gpu = super::web_apis::create_gpu_template(scope, None);
    templates.insert("GPU", tmpl_gpu);
    let tmpl_gpuadapter = super::web_apis::create_gpuadapter_template(scope, None);
    templates.insert("GPUAdapter", tmpl_gpuadapter);
    let tmpl_gpuadapter_info = super::web_apis::create_gpuadapter_info_template(scope, None);
    templates.insert("GPUAdapterInfo", tmpl_gpuadapter_info);
    let tmpl_gpubind_group = super::web_apis::create_gpubind_group_template(scope, None);
    templates.insert("GPUBindGroup", tmpl_gpubind_group);
    let tmpl_gpubind_group_layout = super::web_apis::create_gpubind_group_layout_template(scope, None);
    templates.insert("GPUBindGroupLayout", tmpl_gpubind_group_layout);
    let tmpl_gpubuffer = super::web_apis::create_gpubuffer_template(scope, None);
    templates.insert("GPUBuffer", tmpl_gpubuffer);
    let tmpl_gpucanvas_context = super::web_apis::create_gpucanvas_context_template(scope, None);
    templates.insert("GPUCanvasContext", tmpl_gpucanvas_context);
    let tmpl_gpucommand_buffer = super::web_apis::create_gpucommand_buffer_template(scope, None);
    templates.insert("GPUCommandBuffer", tmpl_gpucommand_buffer);
    let tmpl_gpucommand_encoder = super::web_apis::create_gpucommand_encoder_template(scope, None);
    templates.insert("GPUCommandEncoder", tmpl_gpucommand_encoder);
    let tmpl_gpucompilation_info = super::web_apis::create_gpucompilation_info_template(scope, None);
    templates.insert("GPUCompilationInfo", tmpl_gpucompilation_info);
    let tmpl_gpucompilation_message = super::web_apis::create_gpucompilation_message_template(scope, None);
    templates.insert("GPUCompilationMessage", tmpl_gpucompilation_message);
    let tmpl_gpucompute_pass_encoder = super::web_apis::create_gpucompute_pass_encoder_template(scope, None);
    templates.insert("GPUComputePassEncoder", tmpl_gpucompute_pass_encoder);
    let tmpl_gpucompute_pipeline = super::web_apis::create_gpucompute_pipeline_template(scope, None);
    templates.insert("GPUComputePipeline", tmpl_gpucompute_pipeline);
    let tmpl_gpudevice_lost_info = super::web_apis::create_gpudevice_lost_info_template(scope, None);
    templates.insert("GPUDeviceLostInfo", tmpl_gpudevice_lost_info);
    let tmpl_gpuerror = super::web_apis::create_gpuerror_template(scope, None);
    templates.insert("GPUError", tmpl_gpuerror);
    let tmpl_gpuexternal_texture = super::web_apis::create_gpuexternal_texture_template(scope, None);
    templates.insert("GPUExternalTexture", tmpl_gpuexternal_texture);
    let tmpl_gpupipeline_layout = super::web_apis::create_gpupipeline_layout_template(scope, None);
    templates.insert("GPUPipelineLayout", tmpl_gpupipeline_layout);
    let tmpl_gpuquery_set = super::web_apis::create_gpuquery_set_template(scope, None);
    templates.insert("GPUQuerySet", tmpl_gpuquery_set);
    let tmpl_gpuqueue = super::web_apis::create_gpuqueue_template(scope, None);
    templates.insert("GPUQueue", tmpl_gpuqueue);
    let tmpl_gpurender_bundle = super::web_apis::create_gpurender_bundle_template(scope, None);
    templates.insert("GPURenderBundle", tmpl_gpurender_bundle);
    let tmpl_gpurender_bundle_encoder = super::web_apis::create_gpurender_bundle_encoder_template(scope, None);
    templates.insert("GPURenderBundleEncoder", tmpl_gpurender_bundle_encoder);
    let tmpl_gpurender_pass_encoder = super::web_apis::create_gpurender_pass_encoder_template(scope, None);
    templates.insert("GPURenderPassEncoder", tmpl_gpurender_pass_encoder);
    let tmpl_gpurender_pipeline = super::web_apis::create_gpurender_pipeline_template(scope, None);
    templates.insert("GPURenderPipeline", tmpl_gpurender_pipeline);
    let tmpl_gpusampler = super::web_apis::create_gpusampler_template(scope, None);
    templates.insert("GPUSampler", tmpl_gpusampler);
    let tmpl_gpushader_module = super::web_apis::create_gpushader_module_template(scope, None);
    templates.insert("GPUShaderModule", tmpl_gpushader_module);
    let tmpl_gpusupported_features = super::web_apis::create_gpusupported_features_template(scope, None);
    templates.insert("GPUSupportedFeatures", tmpl_gpusupported_features);
    let tmpl_gpusupported_limits = super::web_apis::create_gpusupported_limits_template(scope, None);
    templates.insert("GPUSupportedLimits", tmpl_gpusupported_limits);
    let tmpl_gputexture = super::web_apis::create_gputexture_template(scope, None);
    templates.insert("GPUTexture", tmpl_gputexture);
    let tmpl_gputexture_view = super::web_apis::create_gputexture_view_template(scope, None);
    templates.insert("GPUTextureView", tmpl_gputexture_view);
    let tmpl_gamepad = super::web_apis::create_gamepad_template(scope, None);
    templates.insert("Gamepad", tmpl_gamepad);
    let tmpl_gamepad_button = super::web_apis::create_gamepad_button_template(scope, None);
    templates.insert("GamepadButton", tmpl_gamepad_button);
    let tmpl_gamepad_haptic_actuator = super::web_apis::create_gamepad_haptic_actuator_template(scope, None);
    templates.insert("GamepadHapticActuator", tmpl_gamepad_haptic_actuator);
    let tmpl_gamepad_pose = super::web_apis::create_gamepad_pose_template(scope, None);
    templates.insert("GamepadPose", tmpl_gamepad_pose);
    let tmpl_geolocation = super::web_apis::create_geolocation_template(scope, None);
    templates.insert("Geolocation", tmpl_geolocation);
    let tmpl_geolocation_coordinates = super::web_apis::create_geolocation_coordinates_template(scope, None);
    templates.insert("GeolocationCoordinates", tmpl_geolocation_coordinates);
    let tmpl_geolocation_position = super::web_apis::create_geolocation_position_template(scope, None);
    templates.insert("GeolocationPosition", tmpl_geolocation_position);
    let tmpl_geolocation_position_error = super::web_apis::create_geolocation_position_error_template(scope, None);
    templates.insert("GeolocationPositionError", tmpl_geolocation_position_error);
    let tmpl_global = super::web_apis::create_global_template(scope, None);
    templates.insert("Global", tmpl_global);
    let tmpl_group_effect = super::web_apis::create_group_effect_template(scope, None);
    templates.insert("GroupEffect", tmpl_group_effect);
    let tmpl_htmlall_collection = super::html_elements::create_htmlall_collection_template(scope, None);
    templates.insert("HTMLAllCollection", tmpl_htmlall_collection);
    let tmpl_htmlcollection = super::html_elements::create_htmlcollection_template(scope, None);
    templates.insert("HTMLCollection", tmpl_htmlcollection);
    let tmpl_handwriting_drawing = super::web_apis::create_handwriting_drawing_template(scope, None);
    templates.insert("HandwritingDrawing", tmpl_handwriting_drawing);
    let tmpl_handwriting_recognizer = super::web_apis::create_handwriting_recognizer_template(scope, None);
    templates.insert("HandwritingRecognizer", tmpl_handwriting_recognizer);
    let tmpl_handwriting_stroke = super::web_apis::create_handwriting_stroke_template(scope, None);
    templates.insert("HandwritingStroke", tmpl_handwriting_stroke);
    let tmpl_headers = super::fetch::create_headers_template(scope, None);
    templates.insert("Headers", tmpl_headers);
    let tmpl_highlight = super::web_apis::create_highlight_template(scope, None);
    templates.insert("Highlight", tmpl_highlight);
    let tmpl_highlight_registry = super::web_apis::create_highlight_registry_template(scope, None);
    templates.insert("HighlightRegistry", tmpl_highlight_registry);
    let tmpl_history = super::web_apis::create_history_template(scope, None);
    templates.insert("History", tmpl_history);
    let tmpl_idbcursor = super::web_apis::create_idbcursor_template(scope, None);
    templates.insert("IDBCursor", tmpl_idbcursor);
    let tmpl_idbfactory = super::web_apis::create_idbfactory_template(scope, None);
    templates.insert("IDBFactory", tmpl_idbfactory);
    let tmpl_idbindex = super::web_apis::create_idbindex_template(scope, None);
    templates.insert("IDBIndex", tmpl_idbindex);
    let tmpl_idbkey_range = super::web_apis::create_idbkey_range_template(scope, None);
    templates.insert("IDBKeyRange", tmpl_idbkey_range);
    let tmpl_idbobject_store = super::web_apis::create_idbobject_store_template(scope, None);
    templates.insert("IDBObjectStore", tmpl_idbobject_store);
    let tmpl_idbrecord = super::web_apis::create_idbrecord_template(scope, None);
    templates.insert("IDBRecord", tmpl_idbrecord);
    let tmpl_identity_provider = super::web_apis::create_identity_provider_template(scope, None);
    templates.insert("IdentityProvider", tmpl_identity_provider);
    let tmpl_idle_deadline = super::web_apis::create_idle_deadline_template(scope, None);
    templates.insert("IdleDeadline", tmpl_idle_deadline);
    let tmpl_image_bitmap = super::web_apis::create_image_bitmap_template(scope, None);
    templates.insert("ImageBitmap", tmpl_image_bitmap);
    let tmpl_image_bitmap_rendering_context = super::web_apis::create_image_bitmap_rendering_context_template(scope, None);
    templates.insert("ImageBitmapRenderingContext", tmpl_image_bitmap_rendering_context);
    let tmpl_image_capture = super::web_apis::create_image_capture_template(scope, None);
    templates.insert("ImageCapture", tmpl_image_capture);
    let tmpl_image_data = super::web_apis::create_image_data_template(scope, None);
    templates.insert("ImageData", tmpl_image_data);
    let tmpl_image_decoder = super::web_apis::create_image_decoder_template(scope, None);
    templates.insert("ImageDecoder", tmpl_image_decoder);
    let tmpl_image_track = super::web_apis::create_image_track_template(scope, None);
    templates.insert("ImageTrack", tmpl_image_track);
    let tmpl_image_track_list = super::web_apis::create_image_track_list_template(scope, None);
    templates.insert("ImageTrackList", tmpl_image_track_list);
    let tmpl_ink = super::web_apis::create_ink_template(scope, None);
    templates.insert("Ink", tmpl_ink);
    let tmpl_input_device_capabilities = super::web_apis::create_input_device_capabilities_template(scope, None);
    templates.insert("InputDeviceCapabilities", tmpl_input_device_capabilities);
    let tmpl_instance = super::web_apis::create_instance_template(scope, None);
    templates.insert("Instance", tmpl_instance);
    let tmpl_intersection_observer = super::observers::create_intersection_observer_template(scope, None);
    templates.insert("IntersectionObserver", tmpl_intersection_observer);
    let tmpl_intersection_observer_entry = super::observers::create_intersection_observer_entry_template(scope, None);
    templates.insert("IntersectionObserverEntry", tmpl_intersection_observer_entry);
    let tmpl_intrinsic_sizes = super::web_apis::create_intrinsic_sizes_template(scope, None);
    templates.insert("IntrinsicSizes", tmpl_intrinsic_sizes);
    let tmpl_khr_parallel_shader_compile = super::web_apis::create_khr_parallel_shader_compile_template(scope, None);
    templates.insert("KHR_parallel_shader_compile", tmpl_khr_parallel_shader_compile);
    let tmpl_keyboard_layout_map = super::web_apis::create_keyboard_layout_map_template(scope, None);
    templates.insert("KeyboardLayoutMap", tmpl_keyboard_layout_map);
    let tmpl_language_detector = super::web_apis::create_language_detector_template(scope, None);
    templates.insert("LanguageDetector", tmpl_language_detector);
    let tmpl_language_model_params = super::web_apis::create_language_model_params_template(scope, None);
    templates.insert("LanguageModelParams", tmpl_language_model_params);
    let tmpl_launch_params = super::web_apis::create_launch_params_template(scope, None);
    templates.insert("LaunchParams", tmpl_launch_params);
    let tmpl_launch_queue = super::web_apis::create_launch_queue_template(scope, None);
    templates.insert("LaunchQueue", tmpl_launch_queue);
    let tmpl_layout_child = super::web_apis::create_layout_child_template(scope, None);
    templates.insert("LayoutChild", tmpl_layout_child);
    let tmpl_layout_constraints = super::web_apis::create_layout_constraints_template(scope, None);
    templates.insert("LayoutConstraints", tmpl_layout_constraints);
    let tmpl_layout_edges = super::web_apis::create_layout_edges_template(scope, None);
    templates.insert("LayoutEdges", tmpl_layout_edges);
    let tmpl_layout_fragment = super::web_apis::create_layout_fragment_template(scope, None);
    templates.insert("LayoutFragment", tmpl_layout_fragment);
    let tmpl_layout_shift_attribution = super::web_apis::create_layout_shift_attribution_template(scope, None);
    templates.insert("LayoutShiftAttribution", tmpl_layout_shift_attribution);
    let tmpl_location = super::web_apis::create_location_template(scope, None);
    templates.insert("Location", tmpl_location);
    let tmpl_lock = super::web_apis::create_lock_template(scope, None);
    templates.insert("Lock", tmpl_lock);
    let tmpl_lock_manager = super::web_apis::create_lock_manager_template(scope, None);
    templates.insert("LockManager", tmpl_lock_manager);
    let tmpl_midiinput_map = super::web_apis::create_midiinput_map_template(scope, None);
    templates.insert("MIDIInputMap", tmpl_midiinput_map);
    let tmpl_midioutput_map = super::web_apis::create_midioutput_map_template(scope, None);
    templates.insert("MIDIOutputMap", tmpl_midioutput_map);
    let tmpl_ml = super::web_apis::create_ml_template(scope, None);
    templates.insert("ML", tmpl_ml);
    let tmpl_mlcontext = super::web_apis::create_mlcontext_template(scope, None);
    templates.insert("MLContext", tmpl_mlcontext);
    let tmpl_mlgraph = super::web_apis::create_mlgraph_template(scope, None);
    templates.insert("MLGraph", tmpl_mlgraph);
    let tmpl_mlgraph_builder = super::web_apis::create_mlgraph_builder_template(scope, None);
    templates.insert("MLGraphBuilder", tmpl_mlgraph_builder);
    let tmpl_mloperand = super::web_apis::create_mloperand_template(scope, None);
    templates.insert("MLOperand", tmpl_mloperand);
    let tmpl_mltensor = super::web_apis::create_mltensor_template(scope, None);
    templates.insert("MLTensor", tmpl_mltensor);
    let tmpl_media_capabilities = super::media_apis::create_media_capabilities_template(scope, None);
    templates.insert("MediaCapabilities", tmpl_media_capabilities);
    let tmpl_media_device_info = super::media_apis::create_media_device_info_template(scope, None);
    templates.insert("MediaDeviceInfo", tmpl_media_device_info);
    let tmpl_media_error = super::media_apis::create_media_error_template(scope, None);
    templates.insert("MediaError", tmpl_media_error);
    let tmpl_media_key_status_map = super::media_apis::create_media_key_status_map_template(scope, None);
    templates.insert("MediaKeyStatusMap", tmpl_media_key_status_map);
    let tmpl_media_key_system_access = super::media_apis::create_media_key_system_access_template(scope, None);
    templates.insert("MediaKeySystemAccess", tmpl_media_key_system_access);
    let tmpl_media_keys = super::media_apis::create_media_keys_template(scope, None);
    templates.insert("MediaKeys", tmpl_media_keys);
    let tmpl_media_list = super::css_om::create_media_list_template(scope, None);
    templates.insert("MediaList", tmpl_media_list);
    let tmpl_media_metadata = super::media_apis::create_media_metadata_template(scope, None);
    templates.insert("MediaMetadata", tmpl_media_metadata);
    let tmpl_media_session = super::media_apis::create_media_session_template(scope, None);
    templates.insert("MediaSession", tmpl_media_session);
    let tmpl_media_source_handle = super::media_apis::create_media_source_handle_template(scope, None);
    templates.insert("MediaSourceHandle", tmpl_media_source_handle);
    let tmpl_media_stream_track_handle = super::media_apis::create_media_stream_track_handle_template(scope, None);
    templates.insert("MediaStreamTrackHandle", tmpl_media_stream_track_handle);
    let tmpl_media_stream_track_processor = super::media_apis::create_media_stream_track_processor_template(scope, None);
    templates.insert("MediaStreamTrackProcessor", tmpl_media_stream_track_processor);
    let tmpl_memory = super::web_apis::create_memory_template(scope, None);
    templates.insert("Memory", tmpl_memory);
    let tmpl_message_channel = super::workers::create_message_channel_template(scope, None);
    templates.insert("MessageChannel", tmpl_message_channel);
    let tmpl_mime_type = super::web_apis::create_mime_type_template(scope, None);
    templates.insert("MimeType", tmpl_mime_type);
    let tmpl_mime_type_array = super::web_apis::create_mime_type_array_template(scope, None);
    templates.insert("MimeTypeArray", tmpl_mime_type_array);
    let tmpl_model_context_client = super::web_apis::create_model_context_client_template(scope, None);
    templates.insert("ModelContextClient", tmpl_model_context_client);
    let tmpl_module = super::web_apis::create_module_template(scope, None);
    templates.insert("Module", tmpl_module);
    let tmpl_mutation_observer = super::dom_core::create_mutation_observer_template(scope, None);
    templates.insert("MutationObserver", tmpl_mutation_observer);
    let tmpl_mutation_record = super::dom_core::create_mutation_record_template(scope, None);
    templates.insert("MutationRecord", tmpl_mutation_record);
    let tmpl_ndefmessage = super::web_apis::create_ndefmessage_template(scope, None);
    templates.insert("NDEFMessage", tmpl_ndefmessage);
    let tmpl_ndefrecord = super::web_apis::create_ndefrecord_template(scope, None);
    templates.insert("NDEFRecord", tmpl_ndefrecord);
    let tmpl_named_flow_map = super::web_apis::create_named_flow_map_template(scope, None);
    templates.insert("NamedFlowMap", tmpl_named_flow_map);
    let tmpl_named_node_map = super::dom_core::create_named_node_map_template(scope, None);
    templates.insert("NamedNodeMap", tmpl_named_node_map);
    let tmpl_navigation_activation = super::web_apis::create_navigation_activation_template(scope, None);
    templates.insert("NavigationActivation", tmpl_navigation_activation);
    let tmpl_navigation_destination = super::web_apis::create_navigation_destination_template(scope, None);
    templates.insert("NavigationDestination", tmpl_navigation_destination);
    let tmpl_navigation_precommit_controller = super::web_apis::create_navigation_precommit_controller_template(scope, None);
    templates.insert("NavigationPrecommitController", tmpl_navigation_precommit_controller);
    let tmpl_navigation_preload_manager = super::web_apis::create_navigation_preload_manager_template(scope, None);
    templates.insert("NavigationPreloadManager", tmpl_navigation_preload_manager);
    let tmpl_navigation_transition = super::web_apis::create_navigation_transition_template(scope, None);
    templates.insert("NavigationTransition", tmpl_navigation_transition);
    let tmpl_navigator = super::web_apis::create_navigator_template(scope, None);
    templates.insert("Navigator", tmpl_navigator);
    let tmpl_navigator_login = super::web_apis::create_navigator_login_template(scope, None);
    templates.insert("NavigatorLogin", tmpl_navigator_login);
    let tmpl_navigator_uadata = super::web_apis::create_navigator_uadata_template(scope, None);
    templates.insert("NavigatorUAData", tmpl_navigator_uadata);
    let tmpl_node_iterator = super::dom_core::create_node_iterator_template(scope, None);
    templates.insert("NodeIterator", tmpl_node_iterator);
    let tmpl_node_list = super::dom_core::create_node_list_template(scope, None);
    templates.insert("NodeList", tmpl_node_list);
    let tmpl_not_restored_reason_details = super::web_apis::create_not_restored_reason_details_template(scope, None);
    templates.insert("NotRestoredReasonDetails", tmpl_not_restored_reason_details);
    let tmpl_not_restored_reasons = super::web_apis::create_not_restored_reasons_template(scope, None);
    templates.insert("NotRestoredReasons", tmpl_not_restored_reasons);
    let tmpl_oes_draw_buffers_indexed = super::web_apis::create_oes_draw_buffers_indexed_template(scope, None);
    templates.insert("OES_draw_buffers_indexed", tmpl_oes_draw_buffers_indexed);
    let tmpl_oes_element_index_uint = super::web_apis::create_oes_element_index_uint_template(scope, None);
    templates.insert("OES_element_index_uint", tmpl_oes_element_index_uint);
    let tmpl_oes_fbo_render_mipmap = super::web_apis::create_oes_fbo_render_mipmap_template(scope, None);
    templates.insert("OES_fbo_render_mipmap", tmpl_oes_fbo_render_mipmap);
    let tmpl_oes_standard_derivatives = super::web_apis::create_oes_standard_derivatives_template(scope, None);
    templates.insert("OES_standard_derivatives", tmpl_oes_standard_derivatives);
    let tmpl_oes_texture_float = super::web_apis::create_oes_texture_float_template(scope, None);
    templates.insert("OES_texture_float", tmpl_oes_texture_float);
    let tmpl_oes_texture_float_linear = super::web_apis::create_oes_texture_float_linear_template(scope, None);
    templates.insert("OES_texture_float_linear", tmpl_oes_texture_float_linear);
    let tmpl_oes_texture_half_float = super::web_apis::create_oes_texture_half_float_template(scope, None);
    templates.insert("OES_texture_half_float", tmpl_oes_texture_half_float);
    let tmpl_oes_texture_half_float_linear = super::web_apis::create_oes_texture_half_float_linear_template(scope, None);
    templates.insert("OES_texture_half_float_linear", tmpl_oes_texture_half_float_linear);
    let tmpl_oes_vertex_array_object = super::web_apis::create_oes_vertex_array_object_template(scope, None);
    templates.insert("OES_vertex_array_object", tmpl_oes_vertex_array_object);
    let tmpl_ovr_multiview2 = super::web_apis::create_ovr_multiview2_template(scope, None);
    templates.insert("OVR_multiview2", tmpl_ovr_multiview2);
    let tmpl_observable = super::web_apis::create_observable_template(scope, None);
    templates.insert("Observable", tmpl_observable);
    let tmpl_offscreen_canvas_rendering_context2_d = super::web_apis::create_offscreen_canvas_rendering_context2_d_template(scope, None);
    templates.insert("OffscreenCanvasRenderingContext2D", tmpl_offscreen_canvas_rendering_context2_d);
    let tmpl_origin = super::web_apis::create_origin_template(scope, None);
    templates.insert("Origin", tmpl_origin);
    let tmpl_paint_rendering_context2_d = super::web_apis::create_paint_rendering_context2_d_template(scope, None);
    templates.insert("PaintRenderingContext2D", tmpl_paint_rendering_context2_d);
    let tmpl_paint_size = super::web_apis::create_paint_size_template(scope, None);
    templates.insert("PaintSize", tmpl_paint_size);
    let tmpl_path2_d = super::web_apis::create_path2_d_template(scope, None);
    templates.insert("Path2D", tmpl_path2_d);
    let tmpl_payment_manager = super::web_apis::create_payment_manager_template(scope, None);
    templates.insert("PaymentManager", tmpl_payment_manager);
    let tmpl_performance_entry = super::web_apis::create_performance_entry_template(scope, None);
    templates.insert("PerformanceEntry", tmpl_performance_entry);
    let tmpl_performance_navigation = super::web_apis::create_performance_navigation_template(scope, None);
    templates.insert("PerformanceNavigation", tmpl_performance_navigation);
    let tmpl_performance_observer = super::observers::create_performance_observer_template(scope, None);
    templates.insert("PerformanceObserver", tmpl_performance_observer);
    let tmpl_performance_observer_entry_list = super::web_apis::create_performance_observer_entry_list_template(scope, None);
    templates.insert("PerformanceObserverEntryList", tmpl_performance_observer_entry_list);
    let tmpl_performance_server_timing = super::web_apis::create_performance_server_timing_template(scope, None);
    templates.insert("PerformanceServerTiming", tmpl_performance_server_timing);
    let tmpl_performance_timing = super::web_apis::create_performance_timing_template(scope, None);
    templates.insert("PerformanceTiming", tmpl_performance_timing);
    let tmpl_performance_timing_confidence = super::web_apis::create_performance_timing_confidence_template(scope, None);
    templates.insert("PerformanceTimingConfidence", tmpl_performance_timing_confidence);
    let tmpl_periodic_sync_manager = super::web_apis::create_periodic_sync_manager_template(scope, None);
    templates.insert("PeriodicSyncManager", tmpl_periodic_sync_manager);
    let tmpl_periodic_wave = super::web_apis::create_periodic_wave_template(scope, None);
    templates.insert("PeriodicWave", tmpl_periodic_wave);
    let tmpl_permissions = super::media_apis::create_permissions_template(scope, None);
    templates.insert("Permissions", tmpl_permissions);
    let tmpl_permissions_policy = super::web_apis::create_permissions_policy_template(scope, None);
    templates.insert("PermissionsPolicy", tmpl_permissions_policy);
    let tmpl_plugin = super::web_apis::create_plugin_template(scope, None);
    templates.insert("Plugin", tmpl_plugin);
    let tmpl_plugin_array = super::web_apis::create_plugin_array_template(scope, None);
    templates.insert("PluginArray", tmpl_plugin_array);
    let tmpl_preference_manager = super::web_apis::create_preference_manager_template(scope, None);
    templates.insert("PreferenceManager", tmpl_preference_manager);
    let tmpl_presentation = super::web_apis::create_presentation_template(scope, None);
    templates.insert("Presentation", tmpl_presentation);
    let tmpl_presentation_receiver = super::web_apis::create_presentation_receiver_template(scope, None);
    templates.insert("PresentationReceiver", tmpl_presentation_receiver);
    let tmpl_pressure_observer = super::observers::create_pressure_observer_template(scope, None);
    templates.insert("PressureObserver", tmpl_pressure_observer);
    let tmpl_pressure_record = super::web_apis::create_pressure_record_template(scope, None);
    templates.insert("PressureRecord", tmpl_pressure_record);
    let tmpl_push_manager = super::web_apis::create_push_manager_template(scope, None);
    templates.insert("PushManager", tmpl_push_manager);
    let tmpl_push_message_data = super::web_apis::create_push_message_data_template(scope, None);
    templates.insert("PushMessageData", tmpl_push_message_data);
    let tmpl_push_subscription = super::web_apis::create_push_subscription_template(scope, None);
    templates.insert("PushSubscription", tmpl_push_subscription);
    let tmpl_push_subscription_options = super::web_apis::create_push_subscription_options_template(scope, None);
    templates.insert("PushSubscriptionOptions", tmpl_push_subscription_options);
    let tmpl_rtccertificate = super::web_apis::create_rtccertificate_template(scope, None);
    templates.insert("RTCCertificate", tmpl_rtccertificate);
    let tmpl_rtcencoded_audio_frame = super::web_apis::create_rtcencoded_audio_frame_template(scope, None);
    templates.insert("RTCEncodedAudioFrame", tmpl_rtcencoded_audio_frame);
    let tmpl_rtcencoded_video_frame = super::web_apis::create_rtcencoded_video_frame_template(scope, None);
    templates.insert("RTCEncodedVideoFrame", tmpl_rtcencoded_video_frame);
    let tmpl_rtcice_candidate = super::web_apis::create_rtcice_candidate_template(scope, None);
    templates.insert("RTCIceCandidate", tmpl_rtcice_candidate);
    let tmpl_rtcice_candidate_pair = super::web_apis::create_rtcice_candidate_pair_template(scope, None);
    templates.insert("RTCIceCandidatePair", tmpl_rtcice_candidate_pair);
    let tmpl_rtcidentity_assertion = super::web_apis::create_rtcidentity_assertion_template(scope, None);
    templates.insert("RTCIdentityAssertion", tmpl_rtcidentity_assertion);
    let tmpl_rtcidentity_provider_registrar = super::web_apis::create_rtcidentity_provider_registrar_template(scope, None);
    templates.insert("RTCIdentityProviderRegistrar", tmpl_rtcidentity_provider_registrar);
    let tmpl_rtcrtp_receiver = super::web_apis::create_rtcrtp_receiver_template(scope, None);
    templates.insert("RTCRtpReceiver", tmpl_rtcrtp_receiver);
    let tmpl_rtcrtp_sframe_encrypter = super::web_apis::create_rtcrtp_sframe_encrypter_template(scope, None);
    templates.insert("RTCRtpSFrameEncrypter", tmpl_rtcrtp_sframe_encrypter);
    let tmpl_rtcrtp_script_transform = super::web_apis::create_rtcrtp_script_transform_template(scope, None);
    templates.insert("RTCRtpScriptTransform", tmpl_rtcrtp_script_transform);
    let tmpl_rtcrtp_sender = super::web_apis::create_rtcrtp_sender_template(scope, None);
    templates.insert("RTCRtpSender", tmpl_rtcrtp_sender);
    let tmpl_rtcrtp_transceiver = super::web_apis::create_rtcrtp_transceiver_template(scope, None);
    templates.insert("RTCRtpTransceiver", tmpl_rtcrtp_transceiver);
    let tmpl_rtcsession_description = super::web_apis::create_rtcsession_description_template(scope, None);
    templates.insert("RTCSessionDescription", tmpl_rtcsession_description);
    let tmpl_rtcstats_report = super::web_apis::create_rtcstats_report_template(scope, None);
    templates.insert("RTCStatsReport", tmpl_rtcstats_report);
    let tmpl_readable_byte_stream_controller = super::streams::create_readable_byte_stream_controller_template(scope, None);
    templates.insert("ReadableByteStreamController", tmpl_readable_byte_stream_controller);
    let tmpl_readable_stream = super::streams::create_readable_stream_template(scope, None);
    templates.insert("ReadableStream", tmpl_readable_stream);
    let tmpl_readable_stream_byobreader = super::streams::create_readable_stream_byobreader_template(scope, None);
    templates.insert("ReadableStreamBYOBReader", tmpl_readable_stream_byobreader);
    let tmpl_readable_stream_byobrequest = super::streams::create_readable_stream_byobrequest_template(scope, None);
    templates.insert("ReadableStreamBYOBRequest", tmpl_readable_stream_byobrequest);
    let tmpl_readable_stream_default_controller = super::streams::create_readable_stream_default_controller_template(scope, None);
    templates.insert("ReadableStreamDefaultController", tmpl_readable_stream_default_controller);
    let tmpl_readable_stream_default_reader = super::streams::create_readable_stream_default_reader_template(scope, None);
    templates.insert("ReadableStreamDefaultReader", tmpl_readable_stream_default_reader);
    let tmpl_reporting_observer = super::observers::create_reporting_observer_template(scope, None);
    templates.insert("ReportingObserver", tmpl_reporting_observer);
    let tmpl_request = super::fetch::create_request_template(scope, None);
    templates.insert("Request", tmpl_request);
    let tmpl_resize_observer = super::observers::create_resize_observer_template(scope, None);
    templates.insert("ResizeObserver", tmpl_resize_observer);
    let tmpl_resize_observer_entry = super::observers::create_resize_observer_entry_template(scope, None);
    templates.insert("ResizeObserverEntry", tmpl_resize_observer_entry);
    let tmpl_resize_observer_size = super::observers::create_resize_observer_size_template(scope, None);
    templates.insert("ResizeObserverSize", tmpl_resize_observer_size);
    let tmpl_response = super::fetch::create_response_template(scope, None);
    templates.insert("Response", tmpl_response);
    let tmpl_restriction_target = super::web_apis::create_restriction_target_template(scope, None);
    templates.insert("RestrictionTarget", tmpl_restriction_target);
    let tmpl_rewriter = super::web_apis::create_rewriter_template(scope, None);
    templates.insert("Rewriter", tmpl_rewriter);
    let tmpl_sframe_encrypter_stream = super::web_apis::create_sframe_encrypter_stream_template(scope, None);
    templates.insert("SFrameEncrypterStream", tmpl_sframe_encrypter_stream);
    let tmpl_svgangle = super::svg::create_svgangle_template(scope, None);
    templates.insert("SVGAngle", tmpl_svgangle);
    let tmpl_svganimated_angle = super::svg::create_svganimated_angle_template(scope, None);
    templates.insert("SVGAnimatedAngle", tmpl_svganimated_angle);
    let tmpl_svganimated_boolean = super::svg::create_svganimated_boolean_template(scope, None);
    templates.insert("SVGAnimatedBoolean", tmpl_svganimated_boolean);
    let tmpl_svganimated_enumeration = super::svg::create_svganimated_enumeration_template(scope, None);
    templates.insert("SVGAnimatedEnumeration", tmpl_svganimated_enumeration);
    let tmpl_svganimated_integer = super::svg::create_svganimated_integer_template(scope, None);
    templates.insert("SVGAnimatedInteger", tmpl_svganimated_integer);
    let tmpl_svganimated_length = super::svg::create_svganimated_length_template(scope, None);
    templates.insert("SVGAnimatedLength", tmpl_svganimated_length);
    let tmpl_svganimated_length_list = super::svg::create_svganimated_length_list_template(scope, None);
    templates.insert("SVGAnimatedLengthList", tmpl_svganimated_length_list);
    let tmpl_svganimated_number = super::svg::create_svganimated_number_template(scope, None);
    templates.insert("SVGAnimatedNumber", tmpl_svganimated_number);
    let tmpl_svganimated_number_list = super::svg::create_svganimated_number_list_template(scope, None);
    templates.insert("SVGAnimatedNumberList", tmpl_svganimated_number_list);
    let tmpl_svganimated_preserve_aspect_ratio = super::svg::create_svganimated_preserve_aspect_ratio_template(scope, None);
    templates.insert("SVGAnimatedPreserveAspectRatio", tmpl_svganimated_preserve_aspect_ratio);
    let tmpl_svganimated_rect = super::svg::create_svganimated_rect_template(scope, None);
    templates.insert("SVGAnimatedRect", tmpl_svganimated_rect);
    let tmpl_svganimated_string = super::svg::create_svganimated_string_template(scope, None);
    templates.insert("SVGAnimatedString", tmpl_svganimated_string);
    let tmpl_svganimated_transform_list = super::svg::create_svganimated_transform_list_template(scope, None);
    templates.insert("SVGAnimatedTransformList", tmpl_svganimated_transform_list);
    let tmpl_svglength = super::svg::create_svglength_template(scope, None);
    templates.insert("SVGLength", tmpl_svglength);
    let tmpl_svglength_list = super::svg::create_svglength_list_template(scope, None);
    templates.insert("SVGLengthList", tmpl_svglength_list);
    let tmpl_svgnumber = super::svg::create_svgnumber_template(scope, None);
    templates.insert("SVGNumber", tmpl_svgnumber);
    let tmpl_svgnumber_list = super::svg::create_svgnumber_list_template(scope, None);
    templates.insert("SVGNumberList", tmpl_svgnumber_list);
    let tmpl_svgpath_segment = super::svg::create_svgpath_segment_template(scope, None);
    templates.insert("SVGPathSegment", tmpl_svgpath_segment);
    let tmpl_svgpoint_list = super::svg::create_svgpoint_list_template(scope, None);
    templates.insert("SVGPointList", tmpl_svgpoint_list);
    let tmpl_svgpreserve_aspect_ratio = super::svg::create_svgpreserve_aspect_ratio_template(scope, None);
    templates.insert("SVGPreserveAspectRatio", tmpl_svgpreserve_aspect_ratio);
    let tmpl_svgstring_list = super::svg::create_svgstring_list_template(scope, None);
    templates.insert("SVGStringList", tmpl_svgstring_list);
    let tmpl_svgtransform = super::svg::create_svgtransform_template(scope, None);
    templates.insert("SVGTransform", tmpl_svgtransform);
    let tmpl_svgtransform_list = super::svg::create_svgtransform_list_template(scope, None);
    templates.insert("SVGTransformList", tmpl_svgtransform_list);
    let tmpl_svgunit_types = super::svg::create_svgunit_types_template(scope, None);
    templates.insert("SVGUnitTypes", tmpl_svgunit_types);
    let tmpl_sanitizer = super::web_apis::create_sanitizer_template(scope, None);
    templates.insert("Sanitizer", tmpl_sanitizer);
    let tmpl_scheduler = super::web_apis::create_scheduler_template(scope, None);
    templates.insert("Scheduler", tmpl_scheduler);
    let tmpl_scheduling = super::web_apis::create_scheduling_template(scope, None);
    templates.insert("Scheduling", tmpl_scheduling);
    let tmpl_screen = super::css_om::create_screen_template(scope, None);
    templates.insert("Screen", tmpl_screen);
    let tmpl_selection = super::web_apis::create_selection_template(scope, None);
    templates.insert("Selection", tmpl_selection);
    let tmpl_speech_grammar = super::web_apis::create_speech_grammar_template(scope, None);
    templates.insert("SpeechGrammar", tmpl_speech_grammar);
    let tmpl_speech_grammar_list = super::web_apis::create_speech_grammar_list_template(scope, None);
    templates.insert("SpeechGrammarList", tmpl_speech_grammar_list);
    let tmpl_speech_recognition_alternative = super::web_apis::create_speech_recognition_alternative_template(scope, None);
    templates.insert("SpeechRecognitionAlternative", tmpl_speech_recognition_alternative);
    let tmpl_speech_recognition_phrase = super::web_apis::create_speech_recognition_phrase_template(scope, None);
    templates.insert("SpeechRecognitionPhrase", tmpl_speech_recognition_phrase);
    let tmpl_speech_recognition_result = super::web_apis::create_speech_recognition_result_template(scope, None);
    templates.insert("SpeechRecognitionResult", tmpl_speech_recognition_result);
    let tmpl_speech_recognition_result_list = super::web_apis::create_speech_recognition_result_list_template(scope, None);
    templates.insert("SpeechRecognitionResultList", tmpl_speech_recognition_result_list);
    let tmpl_speech_synthesis_voice = super::web_apis::create_speech_synthesis_voice_template(scope, None);
    templates.insert("SpeechSynthesisVoice", tmpl_speech_synthesis_voice);
    let tmpl_storage = super::web_apis::create_storage_template(scope, None);
    templates.insert("Storage", tmpl_storage);
    let tmpl_storage_access_handle = super::web_apis::create_storage_access_handle_template(scope, None);
    templates.insert("StorageAccessHandle", tmpl_storage_access_handle);
    let tmpl_storage_bucket = super::web_apis::create_storage_bucket_template(scope, None);
    templates.insert("StorageBucket", tmpl_storage_bucket);
    let tmpl_storage_bucket_manager = super::web_apis::create_storage_bucket_manager_template(scope, None);
    templates.insert("StorageBucketManager", tmpl_storage_bucket_manager);
    let tmpl_storage_manager = super::web_apis::create_storage_manager_template(scope, None);
    templates.insert("StorageManager", tmpl_storage_manager);
    let tmpl_style_property_map_read_only = super::web_apis::create_style_property_map_read_only_template(scope, None);
    templates.insert("StylePropertyMapReadOnly", tmpl_style_property_map_read_only);
    let tmpl_style_sheet = super::css_om::create_style_sheet_template(scope, None);
    templates.insert("StyleSheet", tmpl_style_sheet);
    let tmpl_style_sheet_list = super::css_om::create_style_sheet_list_template(scope, None);
    templates.insert("StyleSheetList", tmpl_style_sheet_list);
    let tmpl_subscriber = super::web_apis::create_subscriber_template(scope, None);
    templates.insert("Subscriber", tmpl_subscriber);
    let tmpl_subtle_crypto = super::crypto::create_subtle_crypto_template(scope, None);
    templates.insert("SubtleCrypto", tmpl_subtle_crypto);
    let tmpl_summarizer = super::web_apis::create_summarizer_template(scope, None);
    templates.insert("Summarizer", tmpl_summarizer);
    let tmpl_sync_manager = super::web_apis::create_sync_manager_template(scope, None);
    templates.insert("SyncManager", tmpl_sync_manager);
    let tmpl_table = super::web_apis::create_table_template(scope, None);
    templates.insert("Table", tmpl_table);
    let tmpl_tag = super::web_apis::create_tag_template(scope, None);
    templates.insert("Tag", tmpl_tag);
    let tmpl_text_decoder = super::web_apis::create_text_decoder_template(scope, None);
    templates.insert("TextDecoder", tmpl_text_decoder);
    let tmpl_text_decoder_stream = super::web_apis::create_text_decoder_stream_template(scope, None);
    templates.insert("TextDecoderStream", tmpl_text_decoder_stream);
    let tmpl_text_detector = super::web_apis::create_text_detector_template(scope, None);
    templates.insert("TextDetector", tmpl_text_detector);
    let tmpl_text_encoder = super::web_apis::create_text_encoder_template(scope, None);
    templates.insert("TextEncoder", tmpl_text_encoder);
    let tmpl_text_encoder_stream = super::web_apis::create_text_encoder_stream_template(scope, None);
    templates.insert("TextEncoderStream", tmpl_text_encoder_stream);
    let tmpl_text_format = super::web_apis::create_text_format_template(scope, None);
    templates.insert("TextFormat", tmpl_text_format);
    let tmpl_text_metrics = super::web_apis::create_text_metrics_template(scope, None);
    templates.insert("TextMetrics", tmpl_text_metrics);
    let tmpl_text_track_cue_list = super::web_apis::create_text_track_cue_list_template(scope, None);
    templates.insert("TextTrackCueList", tmpl_text_track_cue_list);
    let tmpl_time_ranges = super::web_apis::create_time_ranges_template(scope, None);
    templates.insert("TimeRanges", tmpl_time_ranges);
    let tmpl_touch = super::web_apis::create_touch_template(scope, None);
    templates.insert("Touch", tmpl_touch);
    let tmpl_touch_list = super::web_apis::create_touch_list_template(scope, None);
    templates.insert("TouchList", tmpl_touch_list);
    let tmpl_transform_stream = super::streams::create_transform_stream_template(scope, None);
    templates.insert("TransformStream", tmpl_transform_stream);
    let tmpl_transform_stream_default_controller = super::streams::create_transform_stream_default_controller_template(scope, None);
    templates.insert("TransformStreamDefaultController", tmpl_transform_stream_default_controller);
    let tmpl_translator = super::web_apis::create_translator_template(scope, None);
    templates.insert("Translator", tmpl_translator);
    let tmpl_tree_walker = super::dom_core::create_tree_walker_template(scope, None);
    templates.insert("TreeWalker", tmpl_tree_walker);
    let tmpl_trusted_html = super::web_apis::create_trusted_html_template(scope, None);
    templates.insert("TrustedHTML", tmpl_trusted_html);
    let tmpl_trusted_script = super::web_apis::create_trusted_script_template(scope, None);
    templates.insert("TrustedScript", tmpl_trusted_script);
    let tmpl_trusted_script_url = super::web_apis::create_trusted_script_url_template(scope, None);
    templates.insert("TrustedScriptURL", tmpl_trusted_script_url);
    let tmpl_trusted_type_policy = super::web_apis::create_trusted_type_policy_template(scope, None);
    templates.insert("TrustedTypePolicy", tmpl_trusted_type_policy);
    let tmpl_trusted_type_policy_factory = super::web_apis::create_trusted_type_policy_factory_template(scope, None);
    templates.insert("TrustedTypePolicyFactory", tmpl_trusted_type_policy_factory);
    let tmpl_url = super::web_apis::create_url_template(scope, None);
    templates.insert("URL", tmpl_url);
    let tmpl_urlpattern = super::web_apis::create_urlpattern_template(scope, None);
    templates.insert("URLPattern", tmpl_urlpattern);
    let tmpl_urlsearch_params = super::web_apis::create_urlsearch_params_template(scope, None);
    templates.insert("URLSearchParams", tmpl_urlsearch_params);
    let tmpl_usbalternate_interface = super::web_apis::create_usbalternate_interface_template(scope, None);
    templates.insert("USBAlternateInterface", tmpl_usbalternate_interface);
    let tmpl_usbconfiguration = super::web_apis::create_usbconfiguration_template(scope, None);
    templates.insert("USBConfiguration", tmpl_usbconfiguration);
    let tmpl_usbdevice = super::web_apis::create_usbdevice_template(scope, None);
    templates.insert("USBDevice", tmpl_usbdevice);
    let tmpl_usbendpoint = super::web_apis::create_usbendpoint_template(scope, None);
    templates.insert("USBEndpoint", tmpl_usbendpoint);
    let tmpl_usbin_transfer_result = super::web_apis::create_usbin_transfer_result_template(scope, None);
    templates.insert("USBInTransferResult", tmpl_usbin_transfer_result);
    let tmpl_usbinterface = super::web_apis::create_usbinterface_template(scope, None);
    templates.insert("USBInterface", tmpl_usbinterface);
    let tmpl_usbisochronous_in_transfer_packet = super::web_apis::create_usbisochronous_in_transfer_packet_template(scope, None);
    templates.insert("USBIsochronousInTransferPacket", tmpl_usbisochronous_in_transfer_packet);
    let tmpl_usbisochronous_in_transfer_result = super::web_apis::create_usbisochronous_in_transfer_result_template(scope, None);
    templates.insert("USBIsochronousInTransferResult", tmpl_usbisochronous_in_transfer_result);
    let tmpl_usbisochronous_out_transfer_packet = super::web_apis::create_usbisochronous_out_transfer_packet_template(scope, None);
    templates.insert("USBIsochronousOutTransferPacket", tmpl_usbisochronous_out_transfer_packet);
    let tmpl_usbisochronous_out_transfer_result = super::web_apis::create_usbisochronous_out_transfer_result_template(scope, None);
    templates.insert("USBIsochronousOutTransferResult", tmpl_usbisochronous_out_transfer_result);
    let tmpl_usbout_transfer_result = super::web_apis::create_usbout_transfer_result_template(scope, None);
    templates.insert("USBOutTransferResult", tmpl_usbout_transfer_result);
    let tmpl_user_activation = super::web_apis::create_user_activation_template(scope, None);
    templates.insert("UserActivation", tmpl_user_activation);
    let tmpl_vttregion = super::web_apis::create_vttregion_template(scope, None);
    templates.insert("VTTRegion", tmpl_vttregion);
    let tmpl_validity_state = super::web_apis::create_validity_state_template(scope, None);
    templates.insert("ValidityState", tmpl_validity_state);
    let tmpl_video_color_space = super::web_apis::create_video_color_space_template(scope, None);
    templates.insert("VideoColorSpace", tmpl_video_color_space);
    let tmpl_video_frame = super::web_apis::create_video_frame_template(scope, None);
    templates.insert("VideoFrame", tmpl_video_frame);
    let tmpl_video_playback_quality = super::web_apis::create_video_playback_quality_template(scope, None);
    templates.insert("VideoPlaybackQuality", tmpl_video_playback_quality);
    let tmpl_video_track = super::web_apis::create_video_track_template(scope, None);
    templates.insert("VideoTrack", tmpl_video_track);
    let tmpl_video_track_generator = super::web_apis::create_video_track_generator_template(scope, None);
    templates.insert("VideoTrackGenerator", tmpl_video_track_generator);
    let tmpl_view_transition = super::web_apis::create_view_transition_template(scope, None);
    templates.insert("ViewTransition", tmpl_view_transition);
    let tmpl_view_transition_type_set = super::web_apis::create_view_transition_type_set_template(scope, None);
    templates.insert("ViewTransitionTypeSet", tmpl_view_transition_type_set);
    let tmpl_viewport = super::web_apis::create_viewport_template(scope, None);
    templates.insert("Viewport", tmpl_viewport);
    let tmpl_webgl_blend_equation_advanced_coherent = super::web_apis::create_webgl_blend_equation_advanced_coherent_template(scope, None);
    templates.insert("WEBGL_blend_equation_advanced_coherent", tmpl_webgl_blend_equation_advanced_coherent);
    let tmpl_webgl_clip_cull_distance = super::web_apis::create_webgl_clip_cull_distance_template(scope, None);
    templates.insert("WEBGL_clip_cull_distance", tmpl_webgl_clip_cull_distance);
    let tmpl_webgl_color_buffer_float = super::web_apis::create_webgl_color_buffer_float_template(scope, None);
    templates.insert("WEBGL_color_buffer_float", tmpl_webgl_color_buffer_float);
    let tmpl_webgl_compressed_texture_astc = super::web_apis::create_webgl_compressed_texture_astc_template(scope, None);
    templates.insert("WEBGL_compressed_texture_astc", tmpl_webgl_compressed_texture_astc);
    let tmpl_webgl_compressed_texture_etc = super::web_apis::create_webgl_compressed_texture_etc_template(scope, None);
    templates.insert("WEBGL_compressed_texture_etc", tmpl_webgl_compressed_texture_etc);
    let tmpl_webgl_compressed_texture_etc1 = super::web_apis::create_webgl_compressed_texture_etc1_template(scope, None);
    templates.insert("WEBGL_compressed_texture_etc1", tmpl_webgl_compressed_texture_etc1);
    let tmpl_webgl_compressed_texture_pvrtc = super::web_apis::create_webgl_compressed_texture_pvrtc_template(scope, None);
    templates.insert("WEBGL_compressed_texture_pvrtc", tmpl_webgl_compressed_texture_pvrtc);
    let tmpl_webgl_compressed_texture_s3tc = super::web_apis::create_webgl_compressed_texture_s3tc_template(scope, None);
    templates.insert("WEBGL_compressed_texture_s3tc", tmpl_webgl_compressed_texture_s3tc);
    let tmpl_webgl_compressed_texture_s3tc_srgb = super::web_apis::create_webgl_compressed_texture_s3tc_srgb_template(scope, None);
    templates.insert("WEBGL_compressed_texture_s3tc_srgb", tmpl_webgl_compressed_texture_s3tc_srgb);
    let tmpl_webgl_debug_renderer_info = super::web_apis::create_webgl_debug_renderer_info_template(scope, None);
    templates.insert("WEBGL_debug_renderer_info", tmpl_webgl_debug_renderer_info);
    let tmpl_webgl_debug_shaders = super::web_apis::create_webgl_debug_shaders_template(scope, None);
    templates.insert("WEBGL_debug_shaders", tmpl_webgl_debug_shaders);
    let tmpl_webgl_depth_texture = super::web_apis::create_webgl_depth_texture_template(scope, None);
    templates.insert("WEBGL_depth_texture", tmpl_webgl_depth_texture);
    let tmpl_webgl_draw_buffers = super::web_apis::create_webgl_draw_buffers_template(scope, None);
    templates.insert("WEBGL_draw_buffers", tmpl_webgl_draw_buffers);
    let tmpl_webgl_draw_instanced_base_vertex_base_instance = super::web_apis::create_webgl_draw_instanced_base_vertex_base_instance_template(scope, None);
    templates.insert("WEBGL_draw_instanced_base_vertex_base_instance", tmpl_webgl_draw_instanced_base_vertex_base_instance);
    let tmpl_webgl_lose_context = super::web_apis::create_webgl_lose_context_template(scope, None);
    templates.insert("WEBGL_lose_context", tmpl_webgl_lose_context);
    let tmpl_webgl_multi_draw = super::web_apis::create_webgl_multi_draw_template(scope, None);
    templates.insert("WEBGL_multi_draw", tmpl_webgl_multi_draw);
    let tmpl_webgl_multi_draw_instanced_base_vertex_base_instance = super::web_apis::create_webgl_multi_draw_instanced_base_vertex_base_instance_template(scope, None);
    templates.insert("WEBGL_multi_draw_instanced_base_vertex_base_instance", tmpl_webgl_multi_draw_instanced_base_vertex_base_instance);
    let tmpl_webgl_provoking_vertex = super::web_apis::create_webgl_provoking_vertex_template(scope, None);
    templates.insert("WEBGL_provoking_vertex", tmpl_webgl_provoking_vertex);
    let tmpl_wgsllanguage_features = super::web_apis::create_wgsllanguage_features_template(scope, None);
    templates.insert("WGSLLanguageFeatures", tmpl_wgsllanguage_features);
    let tmpl_wake_lock = super::web_apis::create_wake_lock_template(scope, None);
    templates.insert("WakeLock", tmpl_wake_lock);
    let tmpl_web_gl2_rendering_context = super::webgl::create_web_gl2_rendering_context_template(scope, None);
    templates.insert("WebGL2RenderingContext", tmpl_web_gl2_rendering_context);
    let tmpl_web_glactive_info = super::webgl::create_web_glactive_info_template(scope, None);
    templates.insert("WebGLActiveInfo", tmpl_web_glactive_info);
    let tmpl_web_globject = super::webgl::create_web_globject_template(scope, None);
    templates.insert("WebGLObject", tmpl_web_globject);
    let tmpl_web_glrendering_context = super::webgl::create_web_glrendering_context_template(scope, None);
    templates.insert("WebGLRenderingContext", tmpl_web_glrendering_context);
    let tmpl_web_glshader_precision_format = super::webgl::create_web_glshader_precision_format_template(scope, None);
    templates.insert("WebGLShaderPrecisionFormat", tmpl_web_glshader_precision_format);
    let tmpl_web_gluniform_location = super::webgl::create_web_gluniform_location_template(scope, None);
    templates.insert("WebGLUniformLocation", tmpl_web_gluniform_location);
    let tmpl_web_transport = super::web_apis::create_web_transport_template(scope, None);
    templates.insert("WebTransport", tmpl_web_transport);
    let tmpl_web_transport_bidirectional_stream = super::web_apis::create_web_transport_bidirectional_stream_template(scope, None);
    templates.insert("WebTransportBidirectionalStream", tmpl_web_transport_bidirectional_stream);
    let tmpl_web_transport_datagram_duplex_stream = super::web_apis::create_web_transport_datagram_duplex_stream_template(scope, None);
    templates.insert("WebTransportDatagramDuplexStream", tmpl_web_transport_datagram_duplex_stream);
    let tmpl_web_transport_send_group = super::web_apis::create_web_transport_send_group_template(scope, None);
    templates.insert("WebTransportSendGroup", tmpl_web_transport_send_group);
    let tmpl_worker_location = super::workers::create_worker_location_template(scope, None);
    templates.insert("WorkerLocation", tmpl_worker_location);
    let tmpl_worker_navigator = super::workers::create_worker_navigator_template(scope, None);
    templates.insert("WorkerNavigator", tmpl_worker_navigator);
    let tmpl_worklet = super::web_apis::create_worklet_template(scope, None);
    templates.insert("Worklet", tmpl_worklet);
    let tmpl_worklet_animation_effect = super::web_apis::create_worklet_animation_effect_template(scope, None);
    templates.insert("WorkletAnimationEffect", tmpl_worklet_animation_effect);
    let tmpl_worklet_global_scope = super::web_apis::create_worklet_global_scope_template(scope, None);
    templates.insert("WorkletGlobalScope", tmpl_worklet_global_scope);
    let tmpl_worklet_group_effect = super::web_apis::create_worklet_group_effect_template(scope, None);
    templates.insert("WorkletGroupEffect", tmpl_worklet_group_effect);
    let tmpl_writable_stream = super::streams::create_writable_stream_template(scope, None);
    templates.insert("WritableStream", tmpl_writable_stream);
    let tmpl_writable_stream_default_controller = super::streams::create_writable_stream_default_controller_template(scope, None);
    templates.insert("WritableStreamDefaultController", tmpl_writable_stream_default_controller);
    let tmpl_writable_stream_default_writer = super::streams::create_writable_stream_default_writer_template(scope, None);
    templates.insert("WritableStreamDefaultWriter", tmpl_writable_stream_default_writer);
    let tmpl_writer = super::web_apis::create_writer_template(scope, None);
    templates.insert("Writer", tmpl_writer);
    let tmpl_xmlserializer = super::web_apis::create_xmlserializer_template(scope, None);
    templates.insert("XMLSerializer", tmpl_xmlserializer);
    let tmpl_xpath_evaluator = super::web_apis::create_xpath_evaluator_template(scope, None);
    templates.insert("XPathEvaluator", tmpl_xpath_evaluator);
    let tmpl_xpath_expression = super::web_apis::create_xpath_expression_template(scope, None);
    templates.insert("XPathExpression", tmpl_xpath_expression);
    let tmpl_xpath_result = super::web_apis::create_xpath_result_template(scope, None);
    templates.insert("XPathResult", tmpl_xpath_result);
    let tmpl_xranchor = super::webxr::create_xranchor_template(scope, None);
    templates.insert("XRAnchor", tmpl_xranchor);
    let tmpl_xranchor_set = super::webxr::create_xranchor_set_template(scope, None);
    templates.insert("XRAnchorSet", tmpl_xranchor_set);
    let tmpl_xrbody = super::webxr::create_xrbody_template(scope, None);
    templates.insert("XRBody", tmpl_xrbody);
    let tmpl_xrcamera = super::webxr::create_xrcamera_template(scope, None);
    templates.insert("XRCamera", tmpl_xrcamera);
    let tmpl_xrdepth_information = super::webxr::create_xrdepth_information_template(scope, None);
    templates.insert("XRDepthInformation", tmpl_xrdepth_information);
    let tmpl_xrframe = super::webxr::create_xrframe_template(scope, None);
    templates.insert("XRFrame", tmpl_xrframe);
    let tmpl_xrhand = super::webxr::create_xrhand_template(scope, None);
    templates.insert("XRHand", tmpl_xrhand);
    let tmpl_xrhit_test_result = super::webxr::create_xrhit_test_result_template(scope, None);
    templates.insert("XRHitTestResult", tmpl_xrhit_test_result);
    let tmpl_xrhit_test_source = super::webxr::create_xrhit_test_source_template(scope, None);
    templates.insert("XRHitTestSource", tmpl_xrhit_test_source);
    let tmpl_xrinput_source = super::webxr::create_xrinput_source_template(scope, None);
    templates.insert("XRInputSource", tmpl_xrinput_source);
    let tmpl_xrinput_source_array = super::webxr::create_xrinput_source_array_template(scope, None);
    templates.insert("XRInputSourceArray", tmpl_xrinput_source_array);
    let tmpl_xrlight_estimate = super::webxr::create_xrlight_estimate_template(scope, None);
    templates.insert("XRLightEstimate", tmpl_xrlight_estimate);
    let tmpl_xrmedia_binding = super::webxr::create_xrmedia_binding_template(scope, None);
    templates.insert("XRMediaBinding", tmpl_xrmedia_binding);
    let tmpl_xrmesh = super::webxr::create_xrmesh_template(scope, None);
    templates.insert("XRMesh", tmpl_xrmesh);
    let tmpl_xrmesh_set = super::webxr::create_xrmesh_set_template(scope, None);
    templates.insert("XRMeshSet", tmpl_xrmesh_set);
    let tmpl_xrplane = super::webxr::create_xrplane_template(scope, None);
    templates.insert("XRPlane", tmpl_xrplane);
    let tmpl_xrplane_set = super::webxr::create_xrplane_set_template(scope, None);
    templates.insert("XRPlaneSet", tmpl_xrplane_set);
    let tmpl_xrpose = super::webxr::create_xrpose_template(scope, None);
    templates.insert("XRPose", tmpl_xrpose);
    let tmpl_xrray = super::webxr::create_xrray_template(scope, None);
    templates.insert("XRRay", tmpl_xrray);
    let tmpl_xrrender_state = super::webxr::create_xrrender_state_template(scope, None);
    templates.insert("XRRenderState", tmpl_xrrender_state);
    let tmpl_xrrigid_transform = super::webxr::create_xrrigid_transform_template(scope, None);
    templates.insert("XRRigidTransform", tmpl_xrrigid_transform);
    let tmpl_xrsub_image = super::webxr::create_xrsub_image_template(scope, None);
    templates.insert("XRSubImage", tmpl_xrsub_image);
    let tmpl_xrtransient_input_hit_test_result = super::webxr::create_xrtransient_input_hit_test_result_template(scope, None);
    templates.insert("XRTransientInputHitTestResult", tmpl_xrtransient_input_hit_test_result);
    let tmpl_xrtransient_input_hit_test_source = super::webxr::create_xrtransient_input_hit_test_source_template(scope, None);
    templates.insert("XRTransientInputHitTestSource", tmpl_xrtransient_input_hit_test_source);
    let tmpl_xrview = super::webxr::create_xrview_template(scope, None);
    templates.insert("XRView", tmpl_xrview);
    let tmpl_xrviewport = super::webxr::create_xrviewport_template(scope, None);
    templates.insert("XRViewport", tmpl_xrviewport);
    let tmpl_xrweb_glbinding = super::webxr::create_xrweb_glbinding_template(scope, None);
    templates.insert("XRWebGLBinding", tmpl_xrweb_glbinding);
    let tmpl_xsltprocessor = super::web_apis::create_xsltprocessor_template(scope, None);
    templates.insert("XSLTProcessor", tmpl_xsltprocessor);
    let tmpl_webkit_cancel_animation_frame = super::chrome_extensions::create_webkit_cancel_animation_frame_template(scope, None);
    templates.insert("webkitCancelAnimationFrame", tmpl_webkit_cancel_animation_frame);
    let tmpl_webkit_convert_point_from_node_to_page = super::chrome_extensions::create_webkit_convert_point_from_node_to_page_template(scope, None);
    templates.insert("webkitConvertPointFromNodeToPage", tmpl_webkit_convert_point_from_node_to_page);
    let tmpl_webkit_convert_point_from_page_to_node = super::chrome_extensions::create_webkit_convert_point_from_page_to_node_template(scope, None);
    templates.insert("webkitConvertPointFromPageToNode", tmpl_webkit_convert_point_from_page_to_node);
    let tmpl_webkit_get_gamepads = super::chrome_extensions::create_webkit_get_gamepads_template(scope, None);
    templates.insert("webkitGetGamepads", tmpl_webkit_get_gamepads);
    let tmpl_webkit_get_user_media = super::chrome_extensions::create_webkit_get_user_media_template(scope, None);
    templates.insert("webkitGetUserMedia", tmpl_webkit_get_user_media);
    let tmpl_webkit_idbkey_range = super::chrome_extensions::create_webkit_idbkey_range_template(scope, None);
    templates.insert("webkitIDBKeyRange", tmpl_webkit_idbkey_range);
    let tmpl_webkit_indexed_db = super::chrome_extensions::create_webkit_indexed_db_template(scope, None);
    templates.insert("webkitIndexedDB", tmpl_webkit_indexed_db);
    let tmpl_webkit_match_media = super::chrome_extensions::create_webkit_match_media_template(scope, None);
    templates.insert("webkitMatchMedia", tmpl_webkit_match_media);
    let tmpl_webkit_media_stream = super::chrome_extensions::create_webkit_media_stream_template(scope, None);
    templates.insert("webkitMediaStream", tmpl_webkit_media_stream);
    let tmpl_webkit_notifications = super::chrome_extensions::create_webkit_notifications_template(scope, None);
    templates.insert("webkitNotifications", tmpl_webkit_notifications);
    let tmpl_webkit_performance = super::chrome_extensions::create_webkit_performance_template(scope, None);
    templates.insert("webkitPerformance", tmpl_webkit_performance);
    let tmpl_webkit_rtcpeer_connection = super::chrome_extensions::create_webkit_rtcpeer_connection_template(scope, None);
    templates.insert("webkitRTCPeerConnection", tmpl_webkit_rtcpeer_connection);
    let tmpl_webkit_request_animation_frame = super::chrome_extensions::create_webkit_request_animation_frame_template(scope, None);
    templates.insert("webkitRequestAnimationFrame", tmpl_webkit_request_animation_frame);
    let tmpl_webkit_request_file_system = super::chrome_extensions::create_webkit_request_file_system_template(scope, None);
    templates.insert("webkitRequestFileSystem", tmpl_webkit_request_file_system);
    let tmpl_webkit_resolve_local_file_system_url = super::chrome_extensions::create_webkit_resolve_local_file_system_url_template(scope, None);
    templates.insert("webkitResolveLocalFileSystemURL", tmpl_webkit_resolve_local_file_system_url);
    let tmpl_webkit_speech_grammar_list = super::chrome_extensions::create_webkit_speech_grammar_list_template(scope, None);
    templates.insert("webkitSpeechGrammarList", tmpl_webkit_speech_grammar_list);
    let tmpl_webkit_speech_recognition = super::chrome_extensions::create_webkit_speech_recognition_template(scope, None);
    templates.insert("webkitSpeechRecognition", tmpl_webkit_speech_recognition);
    let tmpl_webkit_speech_recognition_error = super::chrome_extensions::create_webkit_speech_recognition_error_template(scope, None);
    templates.insert("webkitSpeechRecognitionError", tmpl_webkit_speech_recognition_error);
    let tmpl_webkit_speech_recognition_event = super::events::create_webkit_speech_recognition_event_template(scope, None);
    templates.insert("webkitSpeechRecognitionEvent", tmpl_webkit_speech_recognition_event);
    let tmpl_webkit_storage_info = super::chrome_extensions::create_webkit_storage_info_template(scope, None);
    templates.insert("webkitStorageInfo", tmpl_webkit_storage_info);
    let tmpl_webkit_url = super::chrome_extensions::create_webkit_url_template(scope, None);
    templates.insert("webkitURL", tmpl_webkit_url);
    let tmpl_task_controller = super::web_apis::create_task_controller_template(scope, templates.get("AbortController").copied());
    templates.insert("TaskController", tmpl_task_controller);
    let tmpl_range = super::dom_core::create_range_template(scope, templates.get("AbstractRange").copied());
    templates.insert("Range", tmpl_range);
    let tmpl_static_range = super::dom_core::create_static_range_template(scope, templates.get("AbstractRange").copied());
    templates.insert("StaticRange", tmpl_static_range);
    let tmpl_keyframe_effect = super::web_apis::create_keyframe_effect_template(scope, templates.get("AnimationEffect").copied());
    templates.insert("KeyframeEffect", tmpl_keyframe_effect);
    let tmpl_document_timeline = super::web_apis::create_document_timeline_template(scope, templates.get("AnimationTimeline").copied());
    templates.insert("DocumentTimeline", tmpl_document_timeline);
    let tmpl_pointer_timeline = super::web_apis::create_pointer_timeline_template(scope, templates.get("AnimationTimeline").copied());
    templates.insert("PointerTimeline", tmpl_pointer_timeline);
    let tmpl_scroll_timeline = super::web_apis::create_scroll_timeline_template(scope, templates.get("AnimationTimeline").copied());
    templates.insert("ScrollTimeline", tmpl_scroll_timeline);
    let tmpl_authenticator_assertion_response = super::web_apis::create_authenticator_assertion_response_template(scope, templates.get("AuthenticatorResponse").copied());
    templates.insert("AuthenticatorAssertionResponse", tmpl_authenticator_assertion_response);
    let tmpl_authenticator_attestation_response = super::web_apis::create_authenticator_attestation_response_template(scope, templates.get("AuthenticatorResponse").copied());
    templates.insert("AuthenticatorAttestationResponse", tmpl_authenticator_attestation_response);
    let tmpl_file = super::web_apis::create_file_template(scope, templates.get("Blob").copied());
    templates.insert("File", tmpl_file);
    let tmpl_cssparser_at_rule = super::css_om::create_cssparser_at_rule_template(scope, templates.get("CSSParserRule").copied());
    templates.insert("CSSParserAtRule", tmpl_cssparser_at_rule);
    let tmpl_cssparser_declaration = super::css_om::create_cssparser_declaration_template(scope, templates.get("CSSParserRule").copied());
    templates.insert("CSSParserDeclaration", tmpl_cssparser_declaration);
    let tmpl_cssparser_qualified_rule = super::css_om::create_cssparser_qualified_rule_template(scope, templates.get("CSSParserRule").copied());
    templates.insert("CSSParserQualifiedRule", tmpl_cssparser_qualified_rule);
    let tmpl_cssparser_block = super::css_om::create_cssparser_block_template(scope, templates.get("CSSParserValue").copied());
    templates.insert("CSSParserBlock", tmpl_cssparser_block);
    let tmpl_cssparser_function = super::css_om::create_cssparser_function_template(scope, templates.get("CSSParserValue").copied());
    templates.insert("CSSParserFunction", tmpl_cssparser_function);
    let tmpl_cssapply_statement_rule = super::css_om::create_cssapply_statement_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSApplyStatementRule", tmpl_cssapply_statement_rule);
    let tmpl_csscolor_profile_rule = super::css_om::create_csscolor_profile_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSColorProfileRule", tmpl_csscolor_profile_rule);
    let tmpl_csscontents_statement_rule = super::css_om::create_csscontents_statement_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSContentsStatementRule", tmpl_csscontents_statement_rule);
    let tmpl_csscounter_style_rule = super::css_om::create_csscounter_style_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSCounterStyleRule", tmpl_csscounter_style_rule);
    let tmpl_csscustom_media_rule = super::css_om::create_csscustom_media_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSCustomMediaRule", tmpl_csscustom_media_rule);
    let tmpl_cssfont_face_rule = super::css_om::create_cssfont_face_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSFontFaceRule", tmpl_cssfont_face_rule);
    let tmpl_cssfont_feature_values_rule = super::css_om::create_cssfont_feature_values_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSFontFeatureValuesRule", tmpl_cssfont_feature_values_rule);
    let tmpl_cssfont_palette_values_rule = super::css_om::create_cssfont_palette_values_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSFontPaletteValuesRule", tmpl_cssfont_palette_values_rule);
    let tmpl_cssfunction_declarations = super::css_om::create_cssfunction_declarations_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSFunctionDeclarations", tmpl_cssfunction_declarations);
    let tmpl_cssgrouping_rule = super::css_om::create_cssgrouping_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSGroupingRule", tmpl_cssgrouping_rule);
    let tmpl_cssimport_rule = super::css_om::create_cssimport_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSImportRule", tmpl_cssimport_rule);
    let tmpl_csskeyframe_rule = super::css_om::create_csskeyframe_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSKeyframeRule", tmpl_csskeyframe_rule);
    let tmpl_csskeyframes_rule = super::css_om::create_csskeyframes_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSKeyframesRule", tmpl_csskeyframes_rule);
    let tmpl_csslayer_statement_rule = super::css_om::create_csslayer_statement_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSLayerStatementRule", tmpl_csslayer_statement_rule);
    let tmpl_cssmargin_rule = super::css_om::create_cssmargin_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSMarginRule", tmpl_cssmargin_rule);
    let tmpl_cssnamespace_rule = super::css_om::create_cssnamespace_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSNamespaceRule", tmpl_cssnamespace_rule);
    let tmpl_cssnested_declarations = super::css_om::create_cssnested_declarations_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSNestedDeclarations", tmpl_cssnested_declarations);
    let tmpl_cssposition_try_rule = super::css_om::create_cssposition_try_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSPositionTryRule", tmpl_cssposition_try_rule);
    let tmpl_cssproperty_rule = super::css_om::create_cssproperty_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSPropertyRule", tmpl_cssproperty_rule);
    let tmpl_cssview_transition_rule = super::css_om::create_cssview_transition_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSViewTransitionRule", tmpl_cssview_transition_rule);
    let tmpl_cssfont_face_descriptors = super::css_om::create_cssfont_face_descriptors_template(scope, templates.get("CSSStyleDeclaration").copied());
    templates.insert("CSSFontFaceDescriptors", tmpl_cssfont_face_descriptors);
    let tmpl_cssfunction_descriptors = super::css_om::create_cssfunction_descriptors_template(scope, templates.get("CSSStyleDeclaration").copied());
    templates.insert("CSSFunctionDescriptors", tmpl_cssfunction_descriptors);
    let tmpl_csspage_descriptors = super::css_om::create_csspage_descriptors_template(scope, templates.get("CSSStyleDeclaration").copied());
    templates.insert("CSSPageDescriptors", tmpl_csspage_descriptors);
    let tmpl_cssposition_try_descriptors = super::css_om::create_cssposition_try_descriptors_template(scope, templates.get("CSSStyleDeclaration").copied());
    templates.insert("CSSPositionTryDescriptors", tmpl_cssposition_try_descriptors);
    let tmpl_cssstyle_properties = super::css_om::create_cssstyle_properties_template(scope, templates.get("CSSStyleDeclaration").copied());
    templates.insert("CSSStyleProperties", tmpl_cssstyle_properties);
    let tmpl_csscolor_value = super::css_om::create_csscolor_value_template(scope, templates.get("CSSStyleValue").copied());
    templates.insert("CSSColorValue", tmpl_csscolor_value);
    let tmpl_cssimage_value = super::css_om::create_cssimage_value_template(scope, templates.get("CSSStyleValue").copied());
    templates.insert("CSSImageValue", tmpl_cssimage_value);
    let tmpl_csskeyword_value = super::css_om::create_csskeyword_value_template(scope, templates.get("CSSStyleValue").copied());
    templates.insert("CSSKeywordValue", tmpl_csskeyword_value);
    let tmpl_cssnumeric_value = super::css_om::create_cssnumeric_value_template(scope, templates.get("CSSStyleValue").copied());
    templates.insert("CSSNumericValue", tmpl_cssnumeric_value);
    let tmpl_csstransform_value = super::css_om::create_csstransform_value_template(scope, templates.get("CSSStyleValue").copied());
    templates.insert("CSSTransformValue", tmpl_csstransform_value);
    let tmpl_cssunparsed_value = super::css_om::create_cssunparsed_value_template(scope, templates.get("CSSStyleValue").copied());
    templates.insert("CSSUnparsedValue", tmpl_cssunparsed_value);
    let tmpl_cssmatrix_component = super::css_om::create_cssmatrix_component_template(scope, templates.get("CSSTransformComponent").copied());
    templates.insert("CSSMatrixComponent", tmpl_cssmatrix_component);
    let tmpl_cssperspective = super::css_om::create_cssperspective_template(scope, templates.get("CSSTransformComponent").copied());
    templates.insert("CSSPerspective", tmpl_cssperspective);
    let tmpl_cssrotate = super::css_om::create_cssrotate_template(scope, templates.get("CSSTransformComponent").copied());
    templates.insert("CSSRotate", tmpl_cssrotate);
    let tmpl_cssscale = super::css_om::create_cssscale_template(scope, templates.get("CSSTransformComponent").copied());
    templates.insert("CSSScale", tmpl_cssscale);
    let tmpl_cssskew = super::css_om::create_cssskew_template(scope, templates.get("CSSTransformComponent").copied());
    templates.insert("CSSSkew", tmpl_cssskew);
    let tmpl_cssskew_x = super::css_om::create_cssskew_x_template(scope, templates.get("CSSTransformComponent").copied());
    templates.insert("CSSSkewX", tmpl_cssskew_x);
    let tmpl_cssskew_y = super::css_om::create_cssskew_y_template(scope, templates.get("CSSTransformComponent").copied());
    templates.insert("CSSSkewY", tmpl_cssskew_y);
    let tmpl_csstranslate = super::css_om::create_csstranslate_template(scope, templates.get("CSSTransformComponent").copied());
    templates.insert("CSSTranslate", tmpl_csstranslate);
    let tmpl_window_client = super::web_apis::create_window_client_template(scope, templates.get("Client").copied());
    templates.insert("WindowClient", tmpl_window_client);
    let tmpl_digital_credential = super::web_apis::create_digital_credential_template(scope, templates.get("Credential").copied());
    templates.insert("DigitalCredential", tmpl_digital_credential);
    let tmpl_federated_credential = super::web_apis::create_federated_credential_template(scope, templates.get("Credential").copied());
    templates.insert("FederatedCredential", tmpl_federated_credential);
    let tmpl_identity_credential = super::web_apis::create_identity_credential_template(scope, templates.get("Credential").copied());
    templates.insert("IdentityCredential", tmpl_identity_credential);
    let tmpl_otpcredential = super::web_apis::create_otpcredential_template(scope, templates.get("Credential").copied());
    templates.insert("OTPCredential", tmpl_otpcredential);
    let tmpl_password_credential = super::web_apis::create_password_credential_template(scope, templates.get("Credential").copied());
    templates.insert("PasswordCredential", tmpl_password_credential);
    let tmpl_public_key_credential = super::web_apis::create_public_key_credential_template(scope, templates.get("Credential").copied());
    templates.insert("PublicKeyCredential", tmpl_public_key_credential);
    let tmpl_gpupipeline_error = super::web_apis::create_gpupipeline_error_template(scope, templates.get("DOMException").copied());
    templates.insert("GPUPipelineError", tmpl_gpupipeline_error);
    let tmpl_identity_credential_error = super::web_apis::create_identity_credential_error_template(scope, templates.get("DOMException").copied());
    templates.insert("IdentityCredentialError", tmpl_identity_credential_error);
    let tmpl_overconstrained_error = super::web_apis::create_overconstrained_error_template(scope, templates.get("DOMException").copied());
    templates.insert("OverconstrainedError", tmpl_overconstrained_error);
    let tmpl_quota_exceeded_error = super::web_apis::create_quota_exceeded_error_template(scope, templates.get("DOMException").copied());
    templates.insert("QuotaExceededError", tmpl_quota_exceeded_error);
    let tmpl_rtcerror = super::web_apis::create_rtcerror_template(scope, templates.get("DOMException").copied());
    templates.insert("RTCError", tmpl_rtcerror);
    let tmpl_web_transport_error = super::web_apis::create_web_transport_error_template(scope, templates.get("DOMException").copied());
    templates.insert("WebTransportError", tmpl_web_transport_error);
    let tmpl_dommatrix = super::dom_core::create_dommatrix_template(scope, templates.get("DOMMatrixReadOnly").copied());
    templates.insert("DOMMatrix", tmpl_dommatrix);
    let tmpl_dompoint = super::dom_core::create_dompoint_template(scope, templates.get("DOMPointReadOnly").copied());
    templates.insert("DOMPoint", tmpl_dompoint);
    let tmpl_domrect = super::dom_core::create_domrect_template(scope, templates.get("DOMRectReadOnly").copied());
    templates.insert("DOMRect", tmpl_domrect);
    let tmpl_animation_event = super::events::create_animation_event_template(scope, templates.get("Event").copied());
    templates.insert("AnimationEvent", tmpl_animation_event);
    let tmpl_animation_playback_event = super::events::create_animation_playback_event_template(scope, templates.get("Event").copied());
    templates.insert("AnimationPlaybackEvent", tmpl_animation_playback_event);
    let tmpl_audio_processing_event = super::events::create_audio_processing_event_template(scope, templates.get("Event").copied());
    templates.insert("AudioProcessingEvent", tmpl_audio_processing_event);
    let tmpl_autofill_event = super::events::create_autofill_event_template(scope, templates.get("Event").copied());
    templates.insert("AutofillEvent", tmpl_autofill_event);
    let tmpl_before_install_prompt_event = super::events::create_before_install_prompt_event_template(scope, templates.get("Event").copied());
    templates.insert("BeforeInstallPromptEvent", tmpl_before_install_prompt_event);
    let tmpl_before_unload_event = super::events::create_before_unload_event_template(scope, templates.get("Event").copied());
    templates.insert("BeforeUnloadEvent", tmpl_before_unload_event);
    let tmpl_blob_event = super::events::create_blob_event_template(scope, templates.get("Event").copied());
    templates.insert("BlobEvent", tmpl_blob_event);
    let tmpl_bluetooth_advertising_event = super::events::create_bluetooth_advertising_event_template(scope, templates.get("Event").copied());
    templates.insert("BluetoothAdvertisingEvent", tmpl_bluetooth_advertising_event);
    let tmpl_buffered_change_event = super::events::create_buffered_change_event_template(scope, templates.get("Event").copied());
    templates.insert("BufferedChangeEvent", tmpl_buffered_change_event);
    let tmpl_capture_action_event = super::events::create_capture_action_event_template(scope, templates.get("Event").copied());
    templates.insert("CaptureActionEvent", tmpl_capture_action_event);
    let tmpl_captured_mouse_event = super::events::create_captured_mouse_event_template(scope, templates.get("Event").copied());
    templates.insert("CapturedMouseEvent", tmpl_captured_mouse_event);
    let tmpl_character_bounds_update_event = super::events::create_character_bounds_update_event_template(scope, templates.get("Event").copied());
    templates.insert("CharacterBoundsUpdateEvent", tmpl_character_bounds_update_event);
    let tmpl_clipboard_change_event = super::events::create_clipboard_change_event_template(scope, templates.get("Event").copied());
    templates.insert("ClipboardChangeEvent", tmpl_clipboard_change_event);
    let tmpl_clipboard_event = super::events::create_clipboard_event_template(scope, templates.get("Event").copied());
    templates.insert("ClipboardEvent", tmpl_clipboard_event);
    let tmpl_close_event = super::events::create_close_event_template(scope, templates.get("Event").copied());
    templates.insert("CloseEvent", tmpl_close_event);
    let tmpl_command_event = super::events::create_command_event_template(scope, templates.get("Event").copied());
    templates.insert("CommandEvent", tmpl_command_event);
    let tmpl_content_visibility_auto_state_change_event = super::events::create_content_visibility_auto_state_change_event_template(scope, templates.get("Event").copied());
    templates.insert("ContentVisibilityAutoStateChangeEvent", tmpl_content_visibility_auto_state_change_event);
    let tmpl_cookie_change_event = super::events::create_cookie_change_event_template(scope, templates.get("Event").copied());
    templates.insert("CookieChangeEvent", tmpl_cookie_change_event);
    let tmpl_custom_event = super::events::create_custom_event_template(scope, templates.get("Event").copied());
    templates.insert("CustomEvent", tmpl_custom_event);
    let tmpl_device_change_event = super::events::create_device_change_event_template(scope, templates.get("Event").copied());
    templates.insert("DeviceChangeEvent", tmpl_device_change_event);
    let tmpl_device_motion_event = super::events::create_device_motion_event_template(scope, templates.get("Event").copied());
    templates.insert("DeviceMotionEvent", tmpl_device_motion_event);
    let tmpl_device_orientation_event = super::events::create_device_orientation_event_template(scope, templates.get("Event").copied());
    templates.insert("DeviceOrientationEvent", tmpl_device_orientation_event);
    let tmpl_document_picture_in_picture_event = super::events::create_document_picture_in_picture_event_template(scope, templates.get("Event").copied());
    templates.insert("DocumentPictureInPictureEvent", tmpl_document_picture_in_picture_event);
    let tmpl_error_event = super::events::create_error_event_template(scope, templates.get("Event").copied());
    templates.insert("ErrorEvent", tmpl_error_event);
    let tmpl_extendable_event = super::events::create_extendable_event_template(scope, templates.get("Event").copied());
    templates.insert("ExtendableEvent", tmpl_extendable_event);
    let tmpl_font_face_set_load_event = super::events::create_font_face_set_load_event_template(scope, templates.get("Event").copied());
    templates.insert("FontFaceSetLoadEvent", tmpl_font_face_set_load_event);
    let tmpl_form_data_event = super::events::create_form_data_event_template(scope, templates.get("Event").copied());
    templates.insert("FormDataEvent", tmpl_form_data_event);
    let tmpl_gpuuncaptured_error_event = super::events::create_gpuuncaptured_error_event_template(scope, templates.get("Event").copied());
    templates.insert("GPUUncapturedErrorEvent", tmpl_gpuuncaptured_error_event);
    let tmpl_gamepad_event = super::events::create_gamepad_event_template(scope, templates.get("Event").copied());
    templates.insert("GamepadEvent", tmpl_gamepad_event);
    let tmpl_hidconnection_event = super::events::create_hidconnection_event_template(scope, templates.get("Event").copied());
    templates.insert("HIDConnectionEvent", tmpl_hidconnection_event);
    let tmpl_hidinput_report_event = super::events::create_hidinput_report_event_template(scope, templates.get("Event").copied());
    templates.insert("HIDInputReportEvent", tmpl_hidinput_report_event);
    let tmpl_hash_change_event = super::events::create_hash_change_event_template(scope, templates.get("Event").copied());
    templates.insert("HashChangeEvent", tmpl_hash_change_event);
    let tmpl_idbversion_change_event = super::events::create_idbversion_change_event_template(scope, templates.get("Event").copied());
    templates.insert("IDBVersionChangeEvent", tmpl_idbversion_change_event);
    let tmpl_key_frame_request_event = super::events::create_key_frame_request_event_template(scope, templates.get("Event").copied());
    templates.insert("KeyFrameRequestEvent", tmpl_key_frame_request_event);
    let tmpl_midiconnection_event = super::events::create_midiconnection_event_template(scope, templates.get("Event").copied());
    templates.insert("MIDIConnectionEvent", tmpl_midiconnection_event);
    let tmpl_midimessage_event = super::events::create_midimessage_event_template(scope, templates.get("Event").copied());
    templates.insert("MIDIMessageEvent", tmpl_midimessage_event);
    let tmpl_media_encrypted_event = super::events::create_media_encrypted_event_template(scope, templates.get("Event").copied());
    templates.insert("MediaEncryptedEvent", tmpl_media_encrypted_event);
    let tmpl_media_key_message_event = super::events::create_media_key_message_event_template(scope, templates.get("Event").copied());
    templates.insert("MediaKeyMessageEvent", tmpl_media_key_message_event);
    let tmpl_media_query_list_event = super::events::create_media_query_list_event_template(scope, templates.get("Event").copied());
    templates.insert("MediaQueryListEvent", tmpl_media_query_list_event);
    let tmpl_media_stream_track_event = super::events::create_media_stream_track_event_template(scope, templates.get("Event").copied());
    templates.insert("MediaStreamTrackEvent", tmpl_media_stream_track_event);
    let tmpl_message_event = super::events::create_message_event_template(scope, templates.get("Event").copied());
    templates.insert("MessageEvent", tmpl_message_event);
    let tmpl_ndefreading_event = super::events::create_ndefreading_event_template(scope, templates.get("Event").copied());
    templates.insert("NDEFReadingEvent", tmpl_ndefreading_event);
    let tmpl_navigate_event = super::events::create_navigate_event_template(scope, templates.get("Event").copied());
    templates.insert("NavigateEvent", tmpl_navigate_event);
    let tmpl_navigation_current_entry_change_event = super::events::create_navigation_current_entry_change_event_template(scope, templates.get("Event").copied());
    templates.insert("NavigationCurrentEntryChangeEvent", tmpl_navigation_current_entry_change_event);
    let tmpl_offline_audio_completion_event = super::events::create_offline_audio_completion_event_template(scope, templates.get("Event").copied());
    templates.insert("OfflineAudioCompletionEvent", tmpl_offline_audio_completion_event);
    let tmpl_page_reveal_event = super::events::create_page_reveal_event_template(scope, templates.get("Event").copied());
    templates.insert("PageRevealEvent", tmpl_page_reveal_event);
    let tmpl_page_swap_event = super::events::create_page_swap_event_template(scope, templates.get("Event").copied());
    templates.insert("PageSwapEvent", tmpl_page_swap_event);
    let tmpl_page_transition_event = super::events::create_page_transition_event_template(scope, templates.get("Event").copied());
    templates.insert("PageTransitionEvent", tmpl_page_transition_event);
    let tmpl_payment_request_update_event = super::events::create_payment_request_update_event_template(scope, templates.get("Event").copied());
    templates.insert("PaymentRequestUpdateEvent", tmpl_payment_request_update_event);
    let tmpl_picture_in_picture_event = super::events::create_picture_in_picture_event_template(scope, templates.get("Event").copied());
    templates.insert("PictureInPictureEvent", tmpl_picture_in_picture_event);
    let tmpl_pop_state_event = super::events::create_pop_state_event_template(scope, templates.get("Event").copied());
    templates.insert("PopStateEvent", tmpl_pop_state_event);
    let tmpl_portal_activate_event = super::events::create_portal_activate_event_template(scope, templates.get("Event").copied());
    templates.insert("PortalActivateEvent", tmpl_portal_activate_event);
    let tmpl_presentation_connection_available_event = super::events::create_presentation_connection_available_event_template(scope, templates.get("Event").copied());
    templates.insert("PresentationConnectionAvailableEvent", tmpl_presentation_connection_available_event);
    let tmpl_presentation_connection_close_event = super::events::create_presentation_connection_close_event_template(scope, templates.get("Event").copied());
    templates.insert("PresentationConnectionCloseEvent", tmpl_presentation_connection_close_event);
    let tmpl_progress_event = super::events::create_progress_event_template(scope, templates.get("Event").copied());
    templates.insert("ProgressEvent", tmpl_progress_event);
    let tmpl_promise_rejection_event = super::events::create_promise_rejection_event_template(scope, templates.get("Event").copied());
    templates.insert("PromiseRejectionEvent", tmpl_promise_rejection_event);
    let tmpl_rtcdtmftone_change_event = super::events::create_rtcdtmftone_change_event_template(scope, templates.get("Event").copied());
    templates.insert("RTCDTMFToneChangeEvent", tmpl_rtcdtmftone_change_event);
    let tmpl_rtcdata_channel_event = super::events::create_rtcdata_channel_event_template(scope, templates.get("Event").copied());
    templates.insert("RTCDataChannelEvent", tmpl_rtcdata_channel_event);
    let tmpl_rtcerror_event = super::events::create_rtcerror_event_template(scope, templates.get("Event").copied());
    templates.insert("RTCErrorEvent", tmpl_rtcerror_event);
    let tmpl_rtcpeer_connection_ice_error_event = super::events::create_rtcpeer_connection_ice_error_event_template(scope, templates.get("Event").copied());
    templates.insert("RTCPeerConnectionIceErrorEvent", tmpl_rtcpeer_connection_ice_error_event);
    let tmpl_rtcpeer_connection_ice_event = super::events::create_rtcpeer_connection_ice_event_template(scope, templates.get("Event").copied());
    templates.insert("RTCPeerConnectionIceEvent", tmpl_rtcpeer_connection_ice_event);
    let tmpl_rtctrack_event = super::events::create_rtctrack_event_template(scope, templates.get("Event").copied());
    templates.insert("RTCTrackEvent", tmpl_rtctrack_event);
    let tmpl_rtctransform_event = super::events::create_rtctransform_event_template(scope, templates.get("Event").copied());
    templates.insert("RTCTransformEvent", tmpl_rtctransform_event);
    let tmpl_sframe_transform_error_event = super::events::create_sframe_transform_error_event_template(scope, templates.get("Event").copied());
    templates.insert("SFrameTransformErrorEvent", tmpl_sframe_transform_error_event);
    let tmpl_security_policy_violation_event = super::events::create_security_policy_violation_event_template(scope, templates.get("Event").copied());
    templates.insert("SecurityPolicyViolationEvent", tmpl_security_policy_violation_event);
    let tmpl_sensor_error_event = super::events::create_sensor_error_event_template(scope, templates.get("Event").copied());
    templates.insert("SensorErrorEvent", tmpl_sensor_error_event);
    let tmpl_snap_event = super::events::create_snap_event_template(scope, templates.get("Event").copied());
    templates.insert("SnapEvent", tmpl_snap_event);
    let tmpl_speech_recognition_error_event = super::events::create_speech_recognition_error_event_template(scope, templates.get("Event").copied());
    templates.insert("SpeechRecognitionErrorEvent", tmpl_speech_recognition_error_event);
    let tmpl_speech_recognition_event = super::events::create_speech_recognition_event_template(scope, templates.get("Event").copied());
    templates.insert("SpeechRecognitionEvent", tmpl_speech_recognition_event);
    let tmpl_speech_synthesis_event = super::events::create_speech_synthesis_event_template(scope, templates.get("Event").copied());
    templates.insert("SpeechSynthesisEvent", tmpl_speech_synthesis_event);
    let tmpl_storage_event = super::events::create_storage_event_template(scope, templates.get("Event").copied());
    templates.insert("StorageEvent", tmpl_storage_event);
    let tmpl_submit_event = super::events::create_submit_event_template(scope, templates.get("Event").copied());
    templates.insert("SubmitEvent", tmpl_submit_event);
    let tmpl_task_priority_change_event = super::events::create_task_priority_change_event_template(scope, templates.get("Event").copied());
    templates.insert("TaskPriorityChangeEvent", tmpl_task_priority_change_event);
    let tmpl_text_format_update_event = super::events::create_text_format_update_event_template(scope, templates.get("Event").copied());
    templates.insert("TextFormatUpdateEvent", tmpl_text_format_update_event);
    let tmpl_text_update_event = super::events::create_text_update_event_template(scope, templates.get("Event").copied());
    templates.insert("TextUpdateEvent", tmpl_text_update_event);
    let tmpl_time_event = super::events::create_time_event_template(scope, templates.get("Event").copied());
    templates.insert("TimeEvent", tmpl_time_event);
    let tmpl_toggle_event = super::events::create_toggle_event_template(scope, templates.get("Event").copied());
    templates.insert("ToggleEvent", tmpl_toggle_event);
    let tmpl_track_event = super::events::create_track_event_template(scope, templates.get("Event").copied());
    templates.insert("TrackEvent", tmpl_track_event);
    let tmpl_transition_event = super::events::create_transition_event_template(scope, templates.get("Event").copied());
    templates.insert("TransitionEvent", tmpl_transition_event);
    let tmpl_uievent = super::events::create_uievent_template(scope, templates.get("Event").copied());
    templates.insert("UIEvent", tmpl_uievent);
    let tmpl_usbconnection_event = super::events::create_usbconnection_event_template(scope, templates.get("Event").copied());
    templates.insert("USBConnectionEvent", tmpl_usbconnection_event);
    let tmpl_value_event = super::events::create_value_event_template(scope, templates.get("Event").copied());
    templates.insert("ValueEvent", tmpl_value_event);
    let tmpl_web_glcontext_event = super::events::create_web_glcontext_event_template(scope, templates.get("Event").copied());
    templates.insert("WebGLContextEvent", tmpl_web_glcontext_event);
    let tmpl_window_controls_overlay_geometry_change_event = super::events::create_window_controls_overlay_geometry_change_event_template(scope, templates.get("Event").copied());
    templates.insert("WindowControlsOverlayGeometryChangeEvent", tmpl_window_controls_overlay_geometry_change_event);
    let tmpl_xrinput_source_event = super::webxr::create_xrinput_source_event_template(scope, templates.get("Event").copied());
    templates.insert("XRInputSourceEvent", tmpl_xrinput_source_event);
    let tmpl_xrinput_sources_change_event = super::webxr::create_xrinput_sources_change_event_template(scope, templates.get("Event").copied());
    templates.insert("XRInputSourcesChangeEvent", tmpl_xrinput_sources_change_event);
    let tmpl_xrlayer_event = super::webxr::create_xrlayer_event_template(scope, templates.get("Event").copied());
    templates.insert("XRLayerEvent", tmpl_xrlayer_event);
    let tmpl_xrreference_space_event = super::webxr::create_xrreference_space_event_template(scope, templates.get("Event").copied());
    templates.insert("XRReferenceSpaceEvent", tmpl_xrreference_space_event);
    let tmpl_xrsession_event = super::webxr::create_xrsession_event_template(scope, templates.get("Event").copied());
    templates.insert("XRSessionEvent", tmpl_xrsession_event);
    let tmpl_xrvisibility_mask_change_event = super::webxr::create_xrvisibility_mask_change_event_template(scope, templates.get("Event").copied());
    templates.insert("XRVisibilityMaskChangeEvent", tmpl_xrvisibility_mask_change_event);
    let tmpl_abort_signal = super::dom_core::create_abort_signal_template(scope, templates.get("EventTarget").copied());
    templates.insert("AbortSignal", tmpl_abort_signal);
    let tmpl_animation = super::web_apis::create_animation_template(scope, templates.get("EventTarget").copied());
    templates.insert("Animation", tmpl_animation);
    let tmpl_audio_decoder = super::web_audio::create_audio_decoder_template(scope, templates.get("EventTarget").copied());
    templates.insert("AudioDecoder", tmpl_audio_decoder);
    let tmpl_audio_encoder = super::web_audio::create_audio_encoder_template(scope, templates.get("EventTarget").copied());
    templates.insert("AudioEncoder", tmpl_audio_encoder);
    let tmpl_audio_node = super::web_audio::create_audio_node_template(scope, templates.get("EventTarget").copied());
    templates.insert("AudioNode", tmpl_audio_node);
    let tmpl_audio_session = super::web_audio::create_audio_session_template(scope, templates.get("EventTarget").copied());
    templates.insert("AudioSession", tmpl_audio_session);
    let tmpl_audio_track_list = super::web_audio::create_audio_track_list_template(scope, templates.get("EventTarget").copied());
    templates.insert("AudioTrackList", tmpl_audio_track_list);
    let tmpl_background_fetch_registration = super::web_apis::create_background_fetch_registration_template(scope, templates.get("EventTarget").copied());
    templates.insert("BackgroundFetchRegistration", tmpl_background_fetch_registration);
    let tmpl_base_audio_context = super::web_audio::create_base_audio_context_template(scope, templates.get("EventTarget").copied());
    templates.insert("BaseAudioContext", tmpl_base_audio_context);
    let tmpl_battery_manager = super::web_apis::create_battery_manager_template(scope, templates.get("EventTarget").copied());
    templates.insert("BatteryManager", tmpl_battery_manager);
    let tmpl_bluetooth = super::bluetooth::create_bluetooth_template(scope, templates.get("EventTarget").copied());
    templates.insert("Bluetooth", tmpl_bluetooth);
    let tmpl_bluetooth_device = super::bluetooth::create_bluetooth_device_template(scope, templates.get("EventTarget").copied());
    templates.insert("BluetoothDevice", tmpl_bluetooth_device);
    let tmpl_bluetooth_remote_gattcharacteristic = super::bluetooth::create_bluetooth_remote_gattcharacteristic_template(scope, templates.get("EventTarget").copied());
    templates.insert("BluetoothRemoteGATTCharacteristic", tmpl_bluetooth_remote_gattcharacteristic);
    let tmpl_bluetooth_remote_gattservice = super::bluetooth::create_bluetooth_remote_gattservice_template(scope, templates.get("EventTarget").copied());
    templates.insert("BluetoothRemoteGATTService", tmpl_bluetooth_remote_gattservice);
    let tmpl_broadcast_channel = super::web_apis::create_broadcast_channel_template(scope, templates.get("EventTarget").copied());
    templates.insert("BroadcastChannel", tmpl_broadcast_channel);
    let tmpl_capture_controller = super::web_apis::create_capture_controller_template(scope, templates.get("EventTarget").copied());
    templates.insert("CaptureController", tmpl_capture_controller);
    let tmpl_clipboard = super::web_apis::create_clipboard_template(scope, templates.get("EventTarget").copied());
    templates.insert("Clipboard", tmpl_clipboard);
    let tmpl_close_watcher = super::web_apis::create_close_watcher_template(scope, templates.get("EventTarget").copied());
    templates.insert("CloseWatcher", tmpl_close_watcher);
    let tmpl_cookie_store = super::web_apis::create_cookie_store_template(scope, templates.get("EventTarget").copied());
    templates.insert("CookieStore", tmpl_cookie_store);
    let tmpl_create_monitor = super::web_apis::create_create_monitor_template(scope, templates.get("EventTarget").copied());
    templates.insert("CreateMonitor", tmpl_create_monitor);
    let tmpl_device_posture = super::web_apis::create_device_posture_template(scope, templates.get("EventTarget").copied());
    templates.insert("DevicePosture", tmpl_device_posture);
    let tmpl_document_picture_in_picture = super::web_apis::create_document_picture_in_picture_template(scope, templates.get("EventTarget").copied());
    templates.insert("DocumentPictureInPicture", tmpl_document_picture_in_picture);
    let tmpl_edit_context = super::web_apis::create_edit_context_template(scope, templates.get("EventTarget").copied());
    templates.insert("EditContext", tmpl_edit_context);
    let tmpl_event_source = super::events::create_event_source_template(scope, templates.get("EventTarget").copied());
    templates.insert("EventSource", tmpl_event_source);
    let tmpl_file_reader = super::web_apis::create_file_reader_template(scope, templates.get("EventTarget").copied());
    templates.insert("FileReader", tmpl_file_reader);
    let tmpl_font_face_set = super::web_apis::create_font_face_set_template(scope, templates.get("EventTarget").copied());
    templates.insert("FontFaceSet", tmpl_font_face_set);
    let tmpl_gpudevice = super::web_apis::create_gpudevice_template(scope, templates.get("EventTarget").copied());
    templates.insert("GPUDevice", tmpl_gpudevice);
    let tmpl_hid = super::web_apis::create_hid_template(scope, templates.get("EventTarget").copied());
    templates.insert("HID", tmpl_hid);
    let tmpl_hiddevice = super::web_apis::create_hiddevice_template(scope, templates.get("EventTarget").copied());
    templates.insert("HIDDevice", tmpl_hiddevice);
    let tmpl_idbdatabase = super::web_apis::create_idbdatabase_template(scope, templates.get("EventTarget").copied());
    templates.insert("IDBDatabase", tmpl_idbdatabase);
    let tmpl_idbrequest = super::web_apis::create_idbrequest_template(scope, templates.get("EventTarget").copied());
    templates.insert("IDBRequest", tmpl_idbrequest);
    let tmpl_idbtransaction = super::web_apis::create_idbtransaction_template(scope, templates.get("EventTarget").copied());
    templates.insert("IDBTransaction", tmpl_idbtransaction);
    let tmpl_idle_detector = super::web_apis::create_idle_detector_template(scope, templates.get("EventTarget").copied());
    templates.insert("IdleDetector", tmpl_idle_detector);
    let tmpl_keyboard = super::web_apis::create_keyboard_template(scope, templates.get("EventTarget").copied());
    templates.insert("Keyboard", tmpl_keyboard);
    let tmpl_language_model = super::web_apis::create_language_model_template(scope, templates.get("EventTarget").copied());
    templates.insert("LanguageModel", tmpl_language_model);
    let tmpl_midiaccess = super::web_apis::create_midiaccess_template(scope, templates.get("EventTarget").copied());
    templates.insert("MIDIAccess", tmpl_midiaccess);
    let tmpl_midiport = super::web_apis::create_midiport_template(scope, templates.get("EventTarget").copied());
    templates.insert("MIDIPort", tmpl_midiport);
    let tmpl_media_devices = super::media_apis::create_media_devices_template(scope, templates.get("EventTarget").copied());
    templates.insert("MediaDevices", tmpl_media_devices);
    let tmpl_media_key_session = super::media_apis::create_media_key_session_template(scope, templates.get("EventTarget").copied());
    templates.insert("MediaKeySession", tmpl_media_key_session);
    let tmpl_media_query_list = super::media_apis::create_media_query_list_template(scope, templates.get("EventTarget").copied());
    templates.insert("MediaQueryList", tmpl_media_query_list);
    let tmpl_media_recorder = super::media_apis::create_media_recorder_template(scope, templates.get("EventTarget").copied());
    templates.insert("MediaRecorder", tmpl_media_recorder);
    let tmpl_media_source = super::media_apis::create_media_source_template(scope, templates.get("EventTarget").copied());
    templates.insert("MediaSource", tmpl_media_source);
    let tmpl_media_stream = super::media_apis::create_media_stream_template(scope, templates.get("EventTarget").copied());
    templates.insert("MediaStream", tmpl_media_stream);
    let tmpl_media_stream_track = super::media_apis::create_media_stream_track_template(scope, templates.get("EventTarget").copied());
    templates.insert("MediaStreamTrack", tmpl_media_stream_track);
    let tmpl_message_port = super::workers::create_message_port_template(scope, templates.get("EventTarget").copied());
    templates.insert("MessagePort", tmpl_message_port);
    let tmpl_model_context = super::web_apis::create_model_context_template(scope, templates.get("EventTarget").copied());
    templates.insert("ModelContext", tmpl_model_context);
    let tmpl_ndefreader = super::web_apis::create_ndefreader_template(scope, templates.get("EventTarget").copied());
    templates.insert("NDEFReader", tmpl_ndefreader);
    let tmpl_named_flow = super::web_apis::create_named_flow_template(scope, templates.get("EventTarget").copied());
    templates.insert("NamedFlow", tmpl_named_flow);
    let tmpl_navigation = super::web_apis::create_navigation_template(scope, templates.get("EventTarget").copied());
    templates.insert("Navigation", tmpl_navigation);
    let tmpl_navigation_history_entry = super::web_apis::create_navigation_history_entry_template(scope, templates.get("EventTarget").copied());
    templates.insert("NavigationHistoryEntry", tmpl_navigation_history_entry);
    let tmpl_navigator_managed_data = super::web_apis::create_navigator_managed_data_template(scope, templates.get("EventTarget").copied());
    templates.insert("NavigatorManagedData", tmpl_navigator_managed_data);
    let tmpl_network_information = super::web_apis::create_network_information_template(scope, templates.get("EventTarget").copied());
    templates.insert("NetworkInformation", tmpl_network_information);
    let tmpl_node = super::dom_core::create_node_template(scope, templates.get("EventTarget").copied());
    templates.insert("Node", tmpl_node);
    let tmpl_notification = super::web_apis::create_notification_template(scope, templates.get("EventTarget").copied());
    templates.insert("Notification", tmpl_notification);
    let tmpl_offscreen_canvas = super::web_apis::create_offscreen_canvas_template(scope, templates.get("EventTarget").copied());
    templates.insert("OffscreenCanvas", tmpl_offscreen_canvas);
    let tmpl_payment_request = super::web_apis::create_payment_request_template(scope, templates.get("EventTarget").copied());
    templates.insert("PaymentRequest", tmpl_payment_request);
    let tmpl_payment_response = super::web_apis::create_payment_response_template(scope, templates.get("EventTarget").copied());
    templates.insert("PaymentResponse", tmpl_payment_response);
    let tmpl_performance = super::web_apis::create_performance_template(scope, templates.get("EventTarget").copied());
    templates.insert("Performance", tmpl_performance);
    let tmpl_permission_status = super::media_apis::create_permission_status_template(scope, templates.get("EventTarget").copied());
    templates.insert("PermissionStatus", tmpl_permission_status);
    let tmpl_picture_in_picture_window = super::web_apis::create_picture_in_picture_window_template(scope, templates.get("EventTarget").copied());
    templates.insert("PictureInPictureWindow", tmpl_picture_in_picture_window);
    let tmpl_portal_host = super::web_apis::create_portal_host_template(scope, templates.get("EventTarget").copied());
    templates.insert("PortalHost", tmpl_portal_host);
    let tmpl_preference_object = super::web_apis::create_preference_object_template(scope, templates.get("EventTarget").copied());
    templates.insert("PreferenceObject", tmpl_preference_object);
    let tmpl_presentation_availability = super::web_apis::create_presentation_availability_template(scope, templates.get("EventTarget").copied());
    templates.insert("PresentationAvailability", tmpl_presentation_availability);
    let tmpl_presentation_connection = super::web_apis::create_presentation_connection_template(scope, templates.get("EventTarget").copied());
    templates.insert("PresentationConnection", tmpl_presentation_connection);
    let tmpl_presentation_connection_list = super::web_apis::create_presentation_connection_list_template(scope, templates.get("EventTarget").copied());
    templates.insert("PresentationConnectionList", tmpl_presentation_connection_list);
    let tmpl_presentation_request = super::web_apis::create_presentation_request_template(scope, templates.get("EventTarget").copied());
    templates.insert("PresentationRequest", tmpl_presentation_request);
    let tmpl_profiler = super::web_apis::create_profiler_template(scope, templates.get("EventTarget").copied());
    templates.insert("Profiler", tmpl_profiler);
    let tmpl_rtcdtmfsender = super::web_apis::create_rtcdtmfsender_template(scope, templates.get("EventTarget").copied());
    templates.insert("RTCDTMFSender", tmpl_rtcdtmfsender);
    let tmpl_rtcdata_channel = super::web_apis::create_rtcdata_channel_template(scope, templates.get("EventTarget").copied());
    templates.insert("RTCDataChannel", tmpl_rtcdata_channel);
    let tmpl_rtcdtls_transport = super::web_apis::create_rtcdtls_transport_template(scope, templates.get("EventTarget").copied());
    templates.insert("RTCDtlsTransport", tmpl_rtcdtls_transport);
    let tmpl_rtcice_transport = super::web_apis::create_rtcice_transport_template(scope, templates.get("EventTarget").copied());
    templates.insert("RTCIceTransport", tmpl_rtcice_transport);
    let tmpl_rtcpeer_connection = super::web_apis::create_rtcpeer_connection_template(scope, templates.get("EventTarget").copied());
    templates.insert("RTCPeerConnection", tmpl_rtcpeer_connection);
    let tmpl_rtcrtp_sframe_decrypter = super::web_apis::create_rtcrtp_sframe_decrypter_template(scope, templates.get("EventTarget").copied());
    templates.insert("RTCRtpSFrameDecrypter", tmpl_rtcrtp_sframe_decrypter);
    let tmpl_rtcrtp_script_transformer = super::web_apis::create_rtcrtp_script_transformer_template(scope, templates.get("EventTarget").copied());
    templates.insert("RTCRtpScriptTransformer", tmpl_rtcrtp_script_transformer);
    let tmpl_rtcsctp_transport = super::web_apis::create_rtcsctp_transport_template(scope, templates.get("EventTarget").copied());
    templates.insert("RTCSctpTransport", tmpl_rtcsctp_transport);
    let tmpl_remote_playback = super::web_apis::create_remote_playback_template(scope, templates.get("EventTarget").copied());
    templates.insert("RemotePlayback", tmpl_remote_playback);
    let tmpl_sframe_decrypter_stream = super::web_apis::create_sframe_decrypter_stream_template(scope, templates.get("EventTarget").copied());
    templates.insert("SFrameDecrypterStream", tmpl_sframe_decrypter_stream);
    let tmpl_screen_details = super::web_apis::create_screen_details_template(scope, templates.get("EventTarget").copied());
    templates.insert("ScreenDetails", tmpl_screen_details);
    let tmpl_screen_orientation = super::web_apis::create_screen_orientation_template(scope, templates.get("EventTarget").copied());
    templates.insert("ScreenOrientation", tmpl_screen_orientation);
    let tmpl_sensor = super::sensors::create_sensor_template(scope, templates.get("EventTarget").copied());
    templates.insert("Sensor", tmpl_sensor);
    let tmpl_serial = super::web_apis::create_serial_template(scope, templates.get("EventTarget").copied());
    templates.insert("Serial", tmpl_serial);
    let tmpl_serial_port = super::web_apis::create_serial_port_template(scope, templates.get("EventTarget").copied());
    templates.insert("SerialPort", tmpl_serial_port);
    let tmpl_service_worker = super::workers::create_service_worker_template(scope, templates.get("EventTarget").copied());
    templates.insert("ServiceWorker", tmpl_service_worker);
    let tmpl_service_worker_container = super::workers::create_service_worker_container_template(scope, templates.get("EventTarget").copied());
    templates.insert("ServiceWorkerContainer", tmpl_service_worker_container);
    let tmpl_service_worker_registration = super::workers::create_service_worker_registration_template(scope, templates.get("EventTarget").copied());
    templates.insert("ServiceWorkerRegistration", tmpl_service_worker_registration);
    let tmpl_shared_worker = super::web_apis::create_shared_worker_template(scope, templates.get("EventTarget").copied());
    templates.insert("SharedWorker", tmpl_shared_worker);
    let tmpl_source_buffer = super::web_apis::create_source_buffer_template(scope, templates.get("EventTarget").copied());
    templates.insert("SourceBuffer", tmpl_source_buffer);
    let tmpl_source_buffer_list = super::web_apis::create_source_buffer_list_template(scope, templates.get("EventTarget").copied());
    templates.insert("SourceBufferList", tmpl_source_buffer_list);
    let tmpl_speech_recognition = super::web_apis::create_speech_recognition_template(scope, templates.get("EventTarget").copied());
    templates.insert("SpeechRecognition", tmpl_speech_recognition);
    let tmpl_speech_synthesis = super::web_apis::create_speech_synthesis_template(scope, templates.get("EventTarget").copied());
    templates.insert("SpeechSynthesis", tmpl_speech_synthesis);
    let tmpl_speech_synthesis_utterance = super::web_apis::create_speech_synthesis_utterance_template(scope, templates.get("EventTarget").copied());
    templates.insert("SpeechSynthesisUtterance", tmpl_speech_synthesis_utterance);
    let tmpl_text_track = super::web_apis::create_text_track_template(scope, templates.get("EventTarget").copied());
    templates.insert("TextTrack", tmpl_text_track);
    let tmpl_text_track_cue = super::web_apis::create_text_track_cue_template(scope, templates.get("EventTarget").copied());
    templates.insert("TextTrackCue", tmpl_text_track_cue);
    let tmpl_text_track_list = super::web_apis::create_text_track_list_template(scope, templates.get("EventTarget").copied());
    templates.insert("TextTrackList", tmpl_text_track_list);
    let tmpl_usb = super::web_apis::create_usb_template(scope, templates.get("EventTarget").copied());
    templates.insert("USB", tmpl_usb);
    let tmpl_video_decoder = super::web_apis::create_video_decoder_template(scope, templates.get("EventTarget").copied());
    templates.insert("VideoDecoder", tmpl_video_decoder);
    let tmpl_video_encoder = super::web_apis::create_video_encoder_template(scope, templates.get("EventTarget").copied());
    templates.insert("VideoEncoder", tmpl_video_encoder);
    let tmpl_video_track_list = super::web_apis::create_video_track_list_template(scope, templates.get("EventTarget").copied());
    templates.insert("VideoTrackList", tmpl_video_track_list);
    let tmpl_virtual_keyboard = super::web_apis::create_virtual_keyboard_template(scope, templates.get("EventTarget").copied());
    templates.insert("VirtualKeyboard", tmpl_virtual_keyboard);
    let tmpl_visual_viewport = super::css_om::create_visual_viewport_template(scope, templates.get("EventTarget").copied());
    templates.insert("VisualViewport", tmpl_visual_viewport);
    let tmpl_wake_lock_sentinel = super::web_apis::create_wake_lock_sentinel_template(scope, templates.get("EventTarget").copied());
    templates.insert("WakeLockSentinel", tmpl_wake_lock_sentinel);
    let tmpl_web_socket = super::web_apis::create_web_socket_template(scope, templates.get("EventTarget").copied());
    templates.insert("WebSocket", tmpl_web_socket);
    let tmpl_window = super::web_apis::create_window_template(scope, templates.get("EventTarget").copied());
    templates.insert("Window", tmpl_window);
    let tmpl_window_controls_overlay = super::web_apis::create_window_controls_overlay_template(scope, templates.get("EventTarget").copied());
    templates.insert("WindowControlsOverlay", tmpl_window_controls_overlay);
    let tmpl_worker = super::workers::create_worker_template(scope, templates.get("EventTarget").copied());
    templates.insert("Worker", tmpl_worker);
    let tmpl_worker_global_scope = super::workers::create_worker_global_scope_template(scope, templates.get("EventTarget").copied());
    templates.insert("WorkerGlobalScope", tmpl_worker_global_scope);
    let tmpl_xmlhttp_request_event_target = super::web_apis::create_xmlhttp_request_event_target_template(scope, templates.get("EventTarget").copied());
    templates.insert("XMLHttpRequestEventTarget", tmpl_xmlhttp_request_event_target);
    let tmpl_xrlayer = super::webxr::create_xrlayer_template(scope, templates.get("EventTarget").copied());
    templates.insert("XRLayer", tmpl_xrlayer);
    let tmpl_xrlight_probe = super::webxr::create_xrlight_probe_template(scope, templates.get("EventTarget").copied());
    templates.insert("XRLightProbe", tmpl_xrlight_probe);
    let tmpl_xrsession = super::webxr::create_xrsession_template(scope, templates.get("EventTarget").copied());
    templates.insert("XRSession", tmpl_xrsession);
    let tmpl_xrspace = super::webxr::create_xrspace_template(scope, templates.get("EventTarget").copied());
    templates.insert("XRSpace", tmpl_xrspace);
    let tmpl_xrsystem = super::webxr::create_xrsystem_template(scope, templates.get("EventTarget").copied());
    templates.insert("XRSystem", tmpl_xrsystem);
    let tmpl_file_system_directory_entry = super::web_apis::create_file_system_directory_entry_template(scope, templates.get("FileSystemEntry").copied());
    templates.insert("FileSystemDirectoryEntry", tmpl_file_system_directory_entry);
    let tmpl_file_system_file_entry = super::web_apis::create_file_system_file_entry_template(scope, templates.get("FileSystemEntry").copied());
    templates.insert("FileSystemFileEntry", tmpl_file_system_file_entry);
    let tmpl_file_system_directory_handle = super::web_apis::create_file_system_directory_handle_template(scope, templates.get("FileSystemHandle").copied());
    templates.insert("FileSystemDirectoryHandle", tmpl_file_system_directory_handle);
    let tmpl_file_system_file_handle = super::web_apis::create_file_system_file_handle_template(scope, templates.get("FileSystemHandle").copied());
    templates.insert("FileSystemFileHandle", tmpl_file_system_file_handle);
    let tmpl_gpuinternal_error = super::web_apis::create_gpuinternal_error_template(scope, templates.get("GPUError").copied());
    templates.insert("GPUInternalError", tmpl_gpuinternal_error);
    let tmpl_gpuout_of_memory_error = super::web_apis::create_gpuout_of_memory_error_template(scope, templates.get("GPUError").copied());
    templates.insert("GPUOutOfMemoryError", tmpl_gpuout_of_memory_error);
    let tmpl_gpuvalidation_error = super::web_apis::create_gpuvalidation_error_template(scope, templates.get("GPUError").copied());
    templates.insert("GPUValidationError", tmpl_gpuvalidation_error);
    let tmpl_sequence_effect = super::web_apis::create_sequence_effect_template(scope, templates.get("GroupEffect").copied());
    templates.insert("SequenceEffect", tmpl_sequence_effect);
    let tmpl_htmlform_controls_collection = super::html_elements::create_htmlform_controls_collection_template(scope, templates.get("HTMLCollection").copied());
    templates.insert("HTMLFormControlsCollection", tmpl_htmlform_controls_collection);
    let tmpl_htmloptions_collection = super::html_elements::create_htmloptions_collection_template(scope, templates.get("HTMLCollection").copied());
    templates.insert("HTMLOptionsCollection", tmpl_htmloptions_collection);
    let tmpl_idbcursor_with_value = super::web_apis::create_idbcursor_with_value_template(scope, templates.get("IDBCursor").copied());
    templates.insert("IDBCursorWithValue", tmpl_idbcursor_with_value);
    let tmpl_input_device_info = super::web_apis::create_input_device_info_template(scope, templates.get("MediaDeviceInfo").copied());
    templates.insert("InputDeviceInfo", tmpl_input_device_info);
    let tmpl_radio_node_list = super::web_apis::create_radio_node_list_template(scope, templates.get("NodeList").copied());
    templates.insert("RadioNodeList", tmpl_radio_node_list);
    let tmpl_largest_contentful_paint = super::web_apis::create_largest_contentful_paint_template(scope, templates.get("PerformanceEntry").copied());
    templates.insert("LargestContentfulPaint", tmpl_largest_contentful_paint);
    let tmpl_layout_shift = super::web_apis::create_layout_shift_template(scope, templates.get("PerformanceEntry").copied());
    templates.insert("LayoutShift", tmpl_layout_shift);
    let tmpl_performance_container_timing = super::web_apis::create_performance_container_timing_template(scope, templates.get("PerformanceEntry").copied());
    templates.insert("PerformanceContainerTiming", tmpl_performance_container_timing);
    let tmpl_performance_element_timing = super::web_apis::create_performance_element_timing_template(scope, templates.get("PerformanceEntry").copied());
    templates.insert("PerformanceElementTiming", tmpl_performance_element_timing);
    let tmpl_performance_event_timing = super::web_apis::create_performance_event_timing_template(scope, templates.get("PerformanceEntry").copied());
    templates.insert("PerformanceEventTiming", tmpl_performance_event_timing);
    let tmpl_performance_long_animation_frame_timing = super::web_apis::create_performance_long_animation_frame_timing_template(scope, templates.get("PerformanceEntry").copied());
    templates.insert("PerformanceLongAnimationFrameTiming", tmpl_performance_long_animation_frame_timing);
    let tmpl_performance_long_task_timing = super::web_apis::create_performance_long_task_timing_template(scope, templates.get("PerformanceEntry").copied());
    templates.insert("PerformanceLongTaskTiming", tmpl_performance_long_task_timing);
    let tmpl_performance_mark = super::web_apis::create_performance_mark_template(scope, templates.get("PerformanceEntry").copied());
    templates.insert("PerformanceMark", tmpl_performance_mark);
    let tmpl_performance_measure = super::web_apis::create_performance_measure_template(scope, templates.get("PerformanceEntry").copied());
    templates.insert("PerformanceMeasure", tmpl_performance_measure);
    let tmpl_performance_paint_timing = super::web_apis::create_performance_paint_timing_template(scope, templates.get("PerformanceEntry").copied());
    templates.insert("PerformancePaintTiming", tmpl_performance_paint_timing);
    let tmpl_performance_resource_timing = super::web_apis::create_performance_resource_timing_template(scope, templates.get("PerformanceEntry").copied());
    templates.insert("PerformanceResourceTiming", tmpl_performance_resource_timing);
    let tmpl_performance_script_timing = super::web_apis::create_performance_script_timing_template(scope, templates.get("PerformanceEntry").copied());
    templates.insert("PerformanceScriptTiming", tmpl_performance_script_timing);
    let tmpl_task_attribution_timing = super::web_apis::create_task_attribution_timing_template(scope, templates.get("PerformanceEntry").copied());
    templates.insert("TaskAttributionTiming", tmpl_task_attribution_timing);
    let tmpl_visibility_state_entry = super::web_apis::create_visibility_state_entry_template(scope, templates.get("PerformanceEntry").copied());
    templates.insert("VisibilityStateEntry", tmpl_visibility_state_entry);
    let tmpl_web_transport_receive_stream = super::web_apis::create_web_transport_receive_stream_template(scope, templates.get("ReadableStream").copied());
    templates.insert("WebTransportReceiveStream", tmpl_web_transport_receive_stream);
    let tmpl_screen_detailed = super::web_apis::create_screen_detailed_template(scope, templates.get("Screen").copied());
    templates.insert("ScreenDetailed", tmpl_screen_detailed);
    let tmpl_style_property_map = super::web_apis::create_style_property_map_template(scope, templates.get("StylePropertyMapReadOnly").copied());
    templates.insert("StylePropertyMap", tmpl_style_property_map);
    let tmpl_cssstyle_sheet = super::css_om::create_cssstyle_sheet_template(scope, templates.get("StyleSheet").copied());
    templates.insert("CSSStyleSheet", tmpl_cssstyle_sheet);
    let tmpl_web_glbuffer = super::webgl::create_web_glbuffer_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLBuffer", tmpl_web_glbuffer);
    let tmpl_web_glframebuffer = super::webgl::create_web_glframebuffer_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLFramebuffer", tmpl_web_glframebuffer);
    let tmpl_web_glprogram = super::webgl::create_web_glprogram_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLProgram", tmpl_web_glprogram);
    let tmpl_web_glquery = super::webgl::create_web_glquery_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLQuery", tmpl_web_glquery);
    let tmpl_web_glrenderbuffer = super::webgl::create_web_glrenderbuffer_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLRenderbuffer", tmpl_web_glrenderbuffer);
    let tmpl_web_glsampler = super::webgl::create_web_glsampler_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLSampler", tmpl_web_glsampler);
    let tmpl_web_glshader = super::webgl::create_web_glshader_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLShader", tmpl_web_glshader);
    let tmpl_web_glsync = super::webgl::create_web_glsync_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLSync", tmpl_web_glsync);
    let tmpl_web_gltexture = super::webgl::create_web_gltexture_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLTexture", tmpl_web_gltexture);
    let tmpl_web_gltimer_query_ext = super::webgl::create_web_gltimer_query_ext_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLTimerQueryEXT", tmpl_web_gltimer_query_ext);
    let tmpl_web_gltransform_feedback = super::webgl::create_web_gltransform_feedback_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLTransformFeedback", tmpl_web_gltransform_feedback);
    let tmpl_web_glvertex_array_object = super::webgl::create_web_glvertex_array_object_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLVertexArrayObject", tmpl_web_glvertex_array_object);
    let tmpl_web_glvertex_array_object_oes = super::webgl::create_web_glvertex_array_object_oes_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLVertexArrayObjectOES", tmpl_web_glvertex_array_object_oes);
    let tmpl_audio_worklet = super::web_audio::create_audio_worklet_template(scope, templates.get("Worklet").copied());
    templates.insert("AudioWorklet", tmpl_audio_worklet);
    let tmpl_animation_worklet_global_scope = super::web_apis::create_animation_worklet_global_scope_template(scope, templates.get("WorkletGlobalScope").copied());
    templates.insert("AnimationWorkletGlobalScope", tmpl_animation_worklet_global_scope);
    let tmpl_audio_worklet_global_scope = super::web_audio::create_audio_worklet_global_scope_template(scope, templates.get("WorkletGlobalScope").copied());
    templates.insert("AudioWorkletGlobalScope", tmpl_audio_worklet_global_scope);
    let tmpl_layout_worklet_global_scope = super::web_apis::create_layout_worklet_global_scope_template(scope, templates.get("WorkletGlobalScope").copied());
    templates.insert("LayoutWorkletGlobalScope", tmpl_layout_worklet_global_scope);
    let tmpl_paint_worklet_global_scope = super::web_apis::create_paint_worklet_global_scope_template(scope, templates.get("WorkletGlobalScope").copied());
    templates.insert("PaintWorkletGlobalScope", tmpl_paint_worklet_global_scope);
    let tmpl_file_system_writable_file_stream = super::web_apis::create_file_system_writable_file_stream_template(scope, templates.get("WritableStream").copied());
    templates.insert("FileSystemWritableFileStream", tmpl_file_system_writable_file_stream);
    let tmpl_web_transport_datagrams_writable = super::web_apis::create_web_transport_datagrams_writable_template(scope, templates.get("WritableStream").copied());
    templates.insert("WebTransportDatagramsWritable", tmpl_web_transport_datagrams_writable);
    let tmpl_web_transport_send_stream = super::web_apis::create_web_transport_send_stream_template(scope, templates.get("WritableStream").copied());
    templates.insert("WebTransportSendStream", tmpl_web_transport_send_stream);
    let tmpl_web_transport_writer = super::web_apis::create_web_transport_writer_template(scope, templates.get("WritableStreamDefaultWriter").copied());
    templates.insert("WebTransportWriter", tmpl_web_transport_writer);
    let tmpl_xrcpudepth_information = super::webxr::create_xrcpudepth_information_template(scope, templates.get("XRDepthInformation").copied());
    templates.insert("XRCPUDepthInformation", tmpl_xrcpudepth_information);
    let tmpl_xrweb_gldepth_information = super::webxr::create_xrweb_gldepth_information_template(scope, templates.get("XRDepthInformation").copied());
    templates.insert("XRWebGLDepthInformation", tmpl_xrweb_gldepth_information);
    let tmpl_xrjoint_pose = super::webxr::create_xrjoint_pose_template(scope, templates.get("XRPose").copied());
    templates.insert("XRJointPose", tmpl_xrjoint_pose);
    let tmpl_xrviewer_pose = super::webxr::create_xrviewer_pose_template(scope, templates.get("XRPose").copied());
    templates.insert("XRViewerPose", tmpl_xrviewer_pose);
    let tmpl_xrweb_glsub_image = super::webxr::create_xrweb_glsub_image_template(scope, templates.get("XRSubImage").copied());
    templates.insert("XRWebGLSubImage", tmpl_xrweb_glsub_image);
    let tmpl_view_timeline = super::web_apis::create_view_timeline_template(scope, templates.get("ScrollTimeline").copied());
    templates.insert("ViewTimeline", tmpl_view_timeline);
    let tmpl_cssapply_block_rule = super::css_om::create_cssapply_block_rule_template(scope, templates.get("CSSGroupingRule").copied());
    templates.insert("CSSApplyBlockRule", tmpl_cssapply_block_rule);
    let tmpl_csscondition_rule = super::css_om::create_csscondition_rule_template(scope, templates.get("CSSGroupingRule").copied());
    templates.insert("CSSConditionRule", tmpl_csscondition_rule);
    let tmpl_csscontents_block_rule = super::css_om::create_csscontents_block_rule_template(scope, templates.get("CSSGroupingRule").copied());
    templates.insert("CSSContentsBlockRule", tmpl_csscontents_block_rule);
    let tmpl_cssfunction_rule = super::css_om::create_cssfunction_rule_template(scope, templates.get("CSSGroupingRule").copied());
    templates.insert("CSSFunctionRule", tmpl_cssfunction_rule);
    let tmpl_csslayer_block_rule = super::css_om::create_csslayer_block_rule_template(scope, templates.get("CSSGroupingRule").copied());
    templates.insert("CSSLayerBlockRule", tmpl_csslayer_block_rule);
    let tmpl_cssmixin_rule = super::css_om::create_cssmixin_rule_template(scope, templates.get("CSSGroupingRule").copied());
    templates.insert("CSSMixinRule", tmpl_cssmixin_rule);
    let tmpl_csspage_rule = super::css_om::create_csspage_rule_template(scope, templates.get("CSSGroupingRule").copied());
    templates.insert("CSSPageRule", tmpl_csspage_rule);
    let tmpl_cssscope_rule = super::css_om::create_cssscope_rule_template(scope, templates.get("CSSGroupingRule").copied());
    templates.insert("CSSScopeRule", tmpl_cssscope_rule);
    let tmpl_cssstarting_style_rule = super::css_om::create_cssstarting_style_rule_template(scope, templates.get("CSSGroupingRule").copied());
    templates.insert("CSSStartingStyleRule", tmpl_cssstarting_style_rule);
    let tmpl_cssstyle_rule = super::css_om::create_cssstyle_rule_template(scope, templates.get("CSSGroupingRule").copied());
    templates.insert("CSSStyleRule", tmpl_cssstyle_rule);
    let tmpl_csssupports_condition_rule = super::css_om::create_csssupports_condition_rule_template(scope, templates.get("CSSGroupingRule").copied());
    templates.insert("CSSSupportsConditionRule", tmpl_csssupports_condition_rule);
    let tmpl_csscolor = super::css_om::create_csscolor_template(scope, templates.get("CSSColorValue").copied());
    templates.insert("CSSColor", tmpl_csscolor);
    let tmpl_csshsl = super::css_om::create_csshsl_template(scope, templates.get("CSSColorValue").copied());
    templates.insert("CSSHSL", tmpl_csshsl);
    let tmpl_csshwb = super::css_om::create_csshwb_template(scope, templates.get("CSSColorValue").copied());
    templates.insert("CSSHWB", tmpl_csshwb);
    let tmpl_csslch = super::css_om::create_csslch_template(scope, templates.get("CSSColorValue").copied());
    templates.insert("CSSLCH", tmpl_csslch);
    let tmpl_csslab = super::css_om::create_csslab_template(scope, templates.get("CSSColorValue").copied());
    templates.insert("CSSLab", tmpl_csslab);
    let tmpl_cssoklch = super::css_om::create_cssoklch_template(scope, templates.get("CSSColorValue").copied());
    templates.insert("CSSOKLCH", tmpl_cssoklch);
    let tmpl_cssoklab = super::css_om::create_cssoklab_template(scope, templates.get("CSSColorValue").copied());
    templates.insert("CSSOKLab", tmpl_cssoklab);
    let tmpl_cssrgb = super::css_om::create_cssrgb_template(scope, templates.get("CSSColorValue").copied());
    templates.insert("CSSRGB", tmpl_cssrgb);
    let tmpl_cssmath_value = super::css_om::create_cssmath_value_template(scope, templates.get("CSSNumericValue").copied());
    templates.insert("CSSMathValue", tmpl_cssmath_value);
    let tmpl_cssunit_value = super::css_om::create_cssunit_value_template(scope, templates.get("CSSNumericValue").copied());
    templates.insert("CSSUnitValue", tmpl_cssunit_value);
    let tmpl_background_fetch_event = super::events::create_background_fetch_event_template(scope, templates.get("ExtendableEvent").copied());
    templates.insert("BackgroundFetchEvent", tmpl_background_fetch_event);
    let tmpl_can_make_payment_event = super::events::create_can_make_payment_event_template(scope, templates.get("ExtendableEvent").copied());
    templates.insert("CanMakePaymentEvent", tmpl_can_make_payment_event);
    let tmpl_content_index_event = super::events::create_content_index_event_template(scope, templates.get("ExtendableEvent").copied());
    templates.insert("ContentIndexEvent", tmpl_content_index_event);
    let tmpl_extendable_cookie_change_event = super::events::create_extendable_cookie_change_event_template(scope, templates.get("ExtendableEvent").copied());
    templates.insert("ExtendableCookieChangeEvent", tmpl_extendable_cookie_change_event);
    let tmpl_extendable_message_event = super::events::create_extendable_message_event_template(scope, templates.get("ExtendableEvent").copied());
    templates.insert("ExtendableMessageEvent", tmpl_extendable_message_event);
    let tmpl_fetch_event = super::events::create_fetch_event_template(scope, templates.get("ExtendableEvent").copied());
    templates.insert("FetchEvent", tmpl_fetch_event);
    let tmpl_install_event = super::events::create_install_event_template(scope, templates.get("ExtendableEvent").copied());
    templates.insert("InstallEvent", tmpl_install_event);
    let tmpl_notification_event = super::events::create_notification_event_template(scope, templates.get("ExtendableEvent").copied());
    templates.insert("NotificationEvent", tmpl_notification_event);
    let tmpl_payment_request_event = super::events::create_payment_request_event_template(scope, templates.get("ExtendableEvent").copied());
    templates.insert("PaymentRequestEvent", tmpl_payment_request_event);
    let tmpl_periodic_sync_event = super::events::create_periodic_sync_event_template(scope, templates.get("ExtendableEvent").copied());
    templates.insert("PeriodicSyncEvent", tmpl_periodic_sync_event);
    let tmpl_push_event = super::events::create_push_event_template(scope, templates.get("ExtendableEvent").copied());
    templates.insert("PushEvent", tmpl_push_event);
    let tmpl_push_subscription_change_event = super::events::create_push_subscription_change_event_template(scope, templates.get("ExtendableEvent").copied());
    templates.insert("PushSubscriptionChangeEvent", tmpl_push_subscription_change_event);
    let tmpl_sync_event = super::events::create_sync_event_template(scope, templates.get("ExtendableEvent").copied());
    templates.insert("SyncEvent", tmpl_sync_event);
    let tmpl_payment_method_change_event = super::events::create_payment_method_change_event_template(scope, templates.get("PaymentRequestUpdateEvent").copied());
    templates.insert("PaymentMethodChangeEvent", tmpl_payment_method_change_event);
    let tmpl_speech_synthesis_error_event = super::events::create_speech_synthesis_error_event_template(scope, templates.get("SpeechSynthesisEvent").copied());
    templates.insert("SpeechSynthesisErrorEvent", tmpl_speech_synthesis_error_event);
    let tmpl_composition_event = super::events::create_composition_event_template(scope, templates.get("UIEvent").copied());
    templates.insert("CompositionEvent", tmpl_composition_event);
    let tmpl_focus_event = super::events::create_focus_event_template(scope, templates.get("UIEvent").copied());
    templates.insert("FocusEvent", tmpl_focus_event);
    let tmpl_input_event = super::events::create_input_event_template(scope, templates.get("UIEvent").copied());
    templates.insert("InputEvent", tmpl_input_event);
    let tmpl_keyboard_event = super::events::create_keyboard_event_template(scope, templates.get("UIEvent").copied());
    templates.insert("KeyboardEvent", tmpl_keyboard_event);
    let tmpl_mouse_event = super::events::create_mouse_event_template(scope, templates.get("UIEvent").copied());
    templates.insert("MouseEvent", tmpl_mouse_event);
    let tmpl_navigation_event = super::events::create_navigation_event_template(scope, templates.get("UIEvent").copied());
    templates.insert("NavigationEvent", tmpl_navigation_event);
    let tmpl_text_event = super::events::create_text_event_template(scope, templates.get("UIEvent").copied());
    templates.insert("TextEvent", tmpl_text_event);
    let tmpl_touch_event = super::events::create_touch_event_template(scope, templates.get("UIEvent").copied());
    templates.insert("TouchEvent", tmpl_touch_event);
    let tmpl_task_signal = super::web_apis::create_task_signal_template(scope, templates.get("AbortSignal").copied());
    templates.insert("TaskSignal", tmpl_task_signal);
    let tmpl_cssanimation = super::css_om::create_cssanimation_template(scope, templates.get("Animation").copied());
    templates.insert("CSSAnimation", tmpl_cssanimation);
    let tmpl_csstransition = super::css_om::create_csstransition_template(scope, templates.get("Animation").copied());
    templates.insert("CSSTransition", tmpl_csstransition);
    let tmpl_shadow_animation = super::web_apis::create_shadow_animation_template(scope, templates.get("Animation").copied());
    templates.insert("ShadowAnimation", tmpl_shadow_animation);
    let tmpl_worklet_animation = super::web_apis::create_worklet_animation_template(scope, templates.get("Animation").copied());
    templates.insert("WorkletAnimation", tmpl_worklet_animation);
    let tmpl_analyser_node = super::web_apis::create_analyser_node_template(scope, templates.get("AudioNode").copied());
    templates.insert("AnalyserNode", tmpl_analyser_node);
    let tmpl_audio_destination_node = super::web_audio::create_audio_destination_node_template(scope, templates.get("AudioNode").copied());
    templates.insert("AudioDestinationNode", tmpl_audio_destination_node);
    let tmpl_audio_scheduled_source_node = super::web_audio::create_audio_scheduled_source_node_template(scope, templates.get("AudioNode").copied());
    templates.insert("AudioScheduledSourceNode", tmpl_audio_scheduled_source_node);
    let tmpl_audio_worklet_node = super::web_audio::create_audio_worklet_node_template(scope, templates.get("AudioNode").copied());
    templates.insert("AudioWorkletNode", tmpl_audio_worklet_node);
    let tmpl_biquad_filter_node = super::web_audio::create_biquad_filter_node_template(scope, templates.get("AudioNode").copied());
    templates.insert("BiquadFilterNode", tmpl_biquad_filter_node);
    let tmpl_channel_merger_node = super::web_apis::create_channel_merger_node_template(scope, templates.get("AudioNode").copied());
    templates.insert("ChannelMergerNode", tmpl_channel_merger_node);
    let tmpl_channel_splitter_node = super::web_apis::create_channel_splitter_node_template(scope, templates.get("AudioNode").copied());
    templates.insert("ChannelSplitterNode", tmpl_channel_splitter_node);
    let tmpl_convolver_node = super::web_apis::create_convolver_node_template(scope, templates.get("AudioNode").copied());
    templates.insert("ConvolverNode", tmpl_convolver_node);
    let tmpl_delay_node = super::web_audio::create_delay_node_template(scope, templates.get("AudioNode").copied());
    templates.insert("DelayNode", tmpl_delay_node);
    let tmpl_dynamics_compressor_node = super::web_apis::create_dynamics_compressor_node_template(scope, templates.get("AudioNode").copied());
    templates.insert("DynamicsCompressorNode", tmpl_dynamics_compressor_node);
    let tmpl_gain_node = super::web_audio::create_gain_node_template(scope, templates.get("AudioNode").copied());
    templates.insert("GainNode", tmpl_gain_node);
    let tmpl_iirfilter_node = super::web_apis::create_iirfilter_node_template(scope, templates.get("AudioNode").copied());
    templates.insert("IIRFilterNode", tmpl_iirfilter_node);
    let tmpl_media_element_audio_source_node = super::media_apis::create_media_element_audio_source_node_template(scope, templates.get("AudioNode").copied());
    templates.insert("MediaElementAudioSourceNode", tmpl_media_element_audio_source_node);
    let tmpl_media_stream_audio_destination_node = super::media_apis::create_media_stream_audio_destination_node_template(scope, templates.get("AudioNode").copied());
    templates.insert("MediaStreamAudioDestinationNode", tmpl_media_stream_audio_destination_node);
    let tmpl_media_stream_audio_source_node = super::media_apis::create_media_stream_audio_source_node_template(scope, templates.get("AudioNode").copied());
    templates.insert("MediaStreamAudioSourceNode", tmpl_media_stream_audio_source_node);
    let tmpl_media_stream_track_audio_source_node = super::media_apis::create_media_stream_track_audio_source_node_template(scope, templates.get("AudioNode").copied());
    templates.insert("MediaStreamTrackAudioSourceNode", tmpl_media_stream_track_audio_source_node);
    let tmpl_panner_node = super::web_apis::create_panner_node_template(scope, templates.get("AudioNode").copied());
    templates.insert("PannerNode", tmpl_panner_node);
    let tmpl_script_processor_node = super::web_apis::create_script_processor_node_template(scope, templates.get("AudioNode").copied());
    templates.insert("ScriptProcessorNode", tmpl_script_processor_node);
    let tmpl_stereo_panner_node = super::web_apis::create_stereo_panner_node_template(scope, templates.get("AudioNode").copied());
    templates.insert("StereoPannerNode", tmpl_stereo_panner_node);
    let tmpl_wave_shaper_node = super::web_apis::create_wave_shaper_node_template(scope, templates.get("AudioNode").copied());
    templates.insert("WaveShaperNode", tmpl_wave_shaper_node);
    let tmpl_audio_context = super::web_audio::create_audio_context_template(scope, templates.get("BaseAudioContext").copied());
    templates.insert("AudioContext", tmpl_audio_context);
    let tmpl_offline_audio_context = super::web_audio::create_offline_audio_context_template(scope, templates.get("BaseAudioContext").copied());
    templates.insert("OfflineAudioContext", tmpl_offline_audio_context);
    let tmpl_idbopen_dbrequest = super::web_apis::create_idbopen_dbrequest_template(scope, templates.get("IDBRequest").copied());
    templates.insert("IDBOpenDBRequest", tmpl_idbopen_dbrequest);
    let tmpl_midiinput = super::web_apis::create_midiinput_template(scope, templates.get("MIDIPort").copied());
    templates.insert("MIDIInput", tmpl_midiinput);
    let tmpl_midioutput = super::web_apis::create_midioutput_template(scope, templates.get("MIDIPort").copied());
    templates.insert("MIDIOutput", tmpl_midioutput);
    let tmpl_managed_media_source = super::web_apis::create_managed_media_source_template(scope, templates.get("MediaSource").copied());
    templates.insert("ManagedMediaSource", tmpl_managed_media_source);
    let tmpl_browser_capture_media_stream_track = super::media_apis::create_browser_capture_media_stream_track_template(scope, templates.get("MediaStreamTrack").copied());
    templates.insert("BrowserCaptureMediaStreamTrack", tmpl_browser_capture_media_stream_track);
    let tmpl_canvas_capture_media_stream_track = super::media_apis::create_canvas_capture_media_stream_track_template(scope, templates.get("MediaStreamTrack").copied());
    templates.insert("CanvasCaptureMediaStreamTrack", tmpl_canvas_capture_media_stream_track);
    let tmpl_attr = super::dom_core::create_attr_template(scope, templates.get("Node").copied());
    templates.insert("Attr", tmpl_attr);
    let tmpl_character_data = super::dom_core::create_character_data_template(scope, templates.get("Node").copied());
    templates.insert("CharacterData", tmpl_character_data);
    let tmpl_document = super::dom_core::create_document_template(scope, templates.get("Node").copied());
    templates.insert("Document", tmpl_document);
    let tmpl_document_fragment = super::dom_core::create_document_fragment_template(scope, templates.get("Node").copied());
    templates.insert("DocumentFragment", tmpl_document_fragment);
    let tmpl_document_type = super::dom_core::create_document_type_template(scope, templates.get("Node").copied());
    templates.insert("DocumentType", tmpl_document_type);
    let tmpl_element = super::dom_core::create_element_template(scope, templates.get("Node").copied());
    templates.insert("Element", tmpl_element);
    let tmpl_bluetooth_lescan_permission_result = super::bluetooth::create_bluetooth_lescan_permission_result_template(scope, templates.get("PermissionStatus").copied());
    templates.insert("BluetoothLEScanPermissionResult", tmpl_bluetooth_lescan_permission_result);
    let tmpl_bluetooth_permission_result = super::bluetooth::create_bluetooth_permission_result_template(scope, templates.get("PermissionStatus").copied());
    templates.insert("BluetoothPermissionResult", tmpl_bluetooth_permission_result);
    let tmpl_usbpermission_result = super::web_apis::create_usbpermission_result_template(scope, templates.get("PermissionStatus").copied());
    templates.insert("USBPermissionResult", tmpl_usbpermission_result);
    let tmpl_xrpermission_status = super::webxr::create_xrpermission_status_template(scope, templates.get("PermissionStatus").copied());
    templates.insert("XRPermissionStatus", tmpl_xrpermission_status);
    let tmpl_accelerometer = super::sensors::create_accelerometer_template(scope, templates.get("Sensor").copied());
    templates.insert("Accelerometer", tmpl_accelerometer);
    let tmpl_ambient_light_sensor = super::sensors::create_ambient_light_sensor_template(scope, templates.get("Sensor").copied());
    templates.insert("AmbientLightSensor", tmpl_ambient_light_sensor);
    let tmpl_gyroscope = super::sensors::create_gyroscope_template(scope, templates.get("Sensor").copied());
    templates.insert("Gyroscope", tmpl_gyroscope);
    let tmpl_magnetometer = super::sensors::create_magnetometer_template(scope, templates.get("Sensor").copied());
    templates.insert("Magnetometer", tmpl_magnetometer);
    let tmpl_orientation_sensor = super::sensors::create_orientation_sensor_template(scope, templates.get("Sensor").copied());
    templates.insert("OrientationSensor", tmpl_orientation_sensor);
    let tmpl_proximity_sensor = super::sensors::create_proximity_sensor_template(scope, templates.get("Sensor").copied());
    templates.insert("ProximitySensor", tmpl_proximity_sensor);
    let tmpl_uncalibrated_magnetometer = super::web_apis::create_uncalibrated_magnetometer_template(scope, templates.get("Sensor").copied());
    templates.insert("UncalibratedMagnetometer", tmpl_uncalibrated_magnetometer);
    let tmpl_managed_source_buffer = super::web_apis::create_managed_source_buffer_template(scope, templates.get("SourceBuffer").copied());
    templates.insert("ManagedSourceBuffer", tmpl_managed_source_buffer);
    let tmpl_data_cue = super::web_apis::create_data_cue_template(scope, templates.get("TextTrackCue").copied());
    templates.insert("DataCue", tmpl_data_cue);
    let tmpl_vttcue = super::web_apis::create_vttcue_template(scope, templates.get("TextTrackCue").copied());
    templates.insert("VTTCue", tmpl_vttcue);
    let tmpl_dedicated_worker_global_scope = super::web_apis::create_dedicated_worker_global_scope_template(scope, templates.get("WorkerGlobalScope").copied());
    templates.insert("DedicatedWorkerGlobalScope", tmpl_dedicated_worker_global_scope);
    let tmpl_rtcidentity_provider_global_scope = super::web_apis::create_rtcidentity_provider_global_scope_template(scope, templates.get("WorkerGlobalScope").copied());
    templates.insert("RTCIdentityProviderGlobalScope", tmpl_rtcidentity_provider_global_scope);
    let tmpl_service_worker_global_scope = super::workers::create_service_worker_global_scope_template(scope, templates.get("WorkerGlobalScope").copied());
    templates.insert("ServiceWorkerGlobalScope", tmpl_service_worker_global_scope);
    let tmpl_shared_worker_global_scope = super::web_apis::create_shared_worker_global_scope_template(scope, templates.get("WorkerGlobalScope").copied());
    templates.insert("SharedWorkerGlobalScope", tmpl_shared_worker_global_scope);
    let tmpl_xmlhttp_request = super::web_apis::create_xmlhttp_request_template(scope, templates.get("XMLHttpRequestEventTarget").copied());
    templates.insert("XMLHttpRequest", tmpl_xmlhttp_request);
    let tmpl_xmlhttp_request_upload = super::web_apis::create_xmlhttp_request_upload_template(scope, templates.get("XMLHttpRequestEventTarget").copied());
    templates.insert("XMLHttpRequestUpload", tmpl_xmlhttp_request_upload);
    let tmpl_xrcomposition_layer = super::webxr::create_xrcomposition_layer_template(scope, templates.get("XRLayer").copied());
    templates.insert("XRCompositionLayer", tmpl_xrcomposition_layer);
    let tmpl_xrweb_gllayer = super::webxr::create_xrweb_gllayer_template(scope, templates.get("XRLayer").copied());
    templates.insert("XRWebGLLayer", tmpl_xrweb_gllayer);
    let tmpl_xrbody_space = super::webxr::create_xrbody_space_template(scope, templates.get("XRSpace").copied());
    templates.insert("XRBodySpace", tmpl_xrbody_space);
    let tmpl_xrjoint_space = super::webxr::create_xrjoint_space_template(scope, templates.get("XRSpace").copied());
    templates.insert("XRJointSpace", tmpl_xrjoint_space);
    let tmpl_xrreference_space = super::webxr::create_xrreference_space_template(scope, templates.get("XRSpace").copied());
    templates.insert("XRReferenceSpace", tmpl_xrreference_space);
    let tmpl_performance_navigation_timing = super::web_apis::create_performance_navigation_timing_template(scope, templates.get("PerformanceResourceTiming").copied());
    templates.insert("PerformanceNavigationTiming", tmpl_performance_navigation_timing);
    let tmpl_csscontainer_rule = super::css_om::create_csscontainer_rule_template(scope, templates.get("CSSConditionRule").copied());
    templates.insert("CSSContainerRule", tmpl_csscontainer_rule);
    let tmpl_cssmedia_rule = super::css_om::create_cssmedia_rule_template(scope, templates.get("CSSConditionRule").copied());
    templates.insert("CSSMediaRule", tmpl_cssmedia_rule);
    let tmpl_csssupports_rule = super::css_om::create_csssupports_rule_template(scope, templates.get("CSSConditionRule").copied());
    templates.insert("CSSSupportsRule", tmpl_csssupports_rule);
    let tmpl_cssmath_clamp = super::css_om::create_cssmath_clamp_template(scope, templates.get("CSSMathValue").copied());
    templates.insert("CSSMathClamp", tmpl_cssmath_clamp);
    let tmpl_cssmath_invert = super::css_om::create_cssmath_invert_template(scope, templates.get("CSSMathValue").copied());
    templates.insert("CSSMathInvert", tmpl_cssmath_invert);
    let tmpl_cssmath_max = super::css_om::create_cssmath_max_template(scope, templates.get("CSSMathValue").copied());
    templates.insert("CSSMathMax", tmpl_cssmath_max);
    let tmpl_cssmath_min = super::css_om::create_cssmath_min_template(scope, templates.get("CSSMathValue").copied());
    templates.insert("CSSMathMin", tmpl_cssmath_min);
    let tmpl_cssmath_negate = super::css_om::create_cssmath_negate_template(scope, templates.get("CSSMathValue").copied());
    templates.insert("CSSMathNegate", tmpl_cssmath_negate);
    let tmpl_cssmath_product = super::css_om::create_cssmath_product_template(scope, templates.get("CSSMathValue").copied());
    templates.insert("CSSMathProduct", tmpl_cssmath_product);
    let tmpl_cssmath_sum = super::css_om::create_cssmath_sum_template(scope, templates.get("CSSMathValue").copied());
    templates.insert("CSSMathSum", tmpl_cssmath_sum);
    let tmpl_background_fetch_update_uievent = super::events::create_background_fetch_update_uievent_template(scope, templates.get("BackgroundFetchEvent").copied());
    templates.insert("BackgroundFetchUpdateUIEvent", tmpl_background_fetch_update_uievent);
    let tmpl_drag_event = super::events::create_drag_event_template(scope, templates.get("MouseEvent").copied());
    templates.insert("DragEvent", tmpl_drag_event);
    let tmpl_pointer_event = super::events::create_pointer_event_template(scope, templates.get("MouseEvent").copied());
    templates.insert("PointerEvent", tmpl_pointer_event);
    let tmpl_wheel_event = super::events::create_wheel_event_template(scope, templates.get("MouseEvent").copied());
    templates.insert("WheelEvent", tmpl_wheel_event);
    let tmpl_audio_buffer_source_node = super::web_audio::create_audio_buffer_source_node_template(scope, templates.get("AudioScheduledSourceNode").copied());
    templates.insert("AudioBufferSourceNode", tmpl_audio_buffer_source_node);
    let tmpl_constant_source_node = super::web_apis::create_constant_source_node_template(scope, templates.get("AudioScheduledSourceNode").copied());
    templates.insert("ConstantSourceNode", tmpl_constant_source_node);
    let tmpl_oscillator_node = super::web_audio::create_oscillator_node_template(scope, templates.get("AudioScheduledSourceNode").copied());
    templates.insert("OscillatorNode", tmpl_oscillator_node);
    let tmpl_webkit_audio_context = super::chrome_extensions::create_webkit_audio_context_template(scope, templates.get("AudioContext").copied());
    templates.insert("webkitAudioContext", tmpl_webkit_audio_context);
    let tmpl_comment = super::dom_core::create_comment_template(scope, templates.get("CharacterData").copied());
    templates.insert("Comment", tmpl_comment);
    let tmpl_processing_instruction = super::web_apis::create_processing_instruction_template(scope, templates.get("CharacterData").copied());
    templates.insert("ProcessingInstruction", tmpl_processing_instruction);
    let tmpl_text = super::dom_core::create_text_template(scope, templates.get("CharacterData").copied());
    templates.insert("Text", tmpl_text);
    let tmpl_xmldocument = super::web_apis::create_xmldocument_template(scope, templates.get("Document").copied());
    templates.insert("XMLDocument", tmpl_xmldocument);
    let tmpl_shadow_root = super::dom_core::create_shadow_root_template(scope, templates.get("DocumentFragment").copied());
    templates.insert("ShadowRoot", tmpl_shadow_root);
    let tmpl_htmlelement = super::html_elements::create_htmlelement_template(scope, templates.get("Element").copied());
    templates.insert("HTMLElement", tmpl_htmlelement);
    let tmpl_math_mlelement = super::web_apis::create_math_mlelement_template(scope, templates.get("Element").copied());
    templates.insert("MathMLElement", tmpl_math_mlelement);
    let tmpl_svgelement = super::svg::create_svgelement_template(scope, templates.get("Element").copied());
    templates.insert("SVGElement", tmpl_svgelement);
    let tmpl_gravity_sensor = super::sensors::create_gravity_sensor_template(scope, templates.get("Accelerometer").copied());
    templates.insert("GravitySensor", tmpl_gravity_sensor);
    let tmpl_linear_acceleration_sensor = super::sensors::create_linear_acceleration_sensor_template(scope, templates.get("Accelerometer").copied());
    templates.insert("LinearAccelerationSensor", tmpl_linear_acceleration_sensor);
    let tmpl_absolute_orientation_sensor = super::sensors::create_absolute_orientation_sensor_template(scope, templates.get("OrientationSensor").copied());
    templates.insert("AbsoluteOrientationSensor", tmpl_absolute_orientation_sensor);
    let tmpl_relative_orientation_sensor = super::sensors::create_relative_orientation_sensor_template(scope, templates.get("OrientationSensor").copied());
    templates.insert("RelativeOrientationSensor", tmpl_relative_orientation_sensor);
    let tmpl_xrcube_layer = super::webxr::create_xrcube_layer_template(scope, templates.get("XRCompositionLayer").copied());
    templates.insert("XRCubeLayer", tmpl_xrcube_layer);
    let tmpl_xrcylinder_layer = super::webxr::create_xrcylinder_layer_template(scope, templates.get("XRCompositionLayer").copied());
    templates.insert("XRCylinderLayer", tmpl_xrcylinder_layer);
    let tmpl_xrequirect_layer = super::webxr::create_xrequirect_layer_template(scope, templates.get("XRCompositionLayer").copied());
    templates.insert("XREquirectLayer", tmpl_xrequirect_layer);
    let tmpl_xrprojection_layer = super::webxr::create_xrprojection_layer_template(scope, templates.get("XRCompositionLayer").copied());
    templates.insert("XRProjectionLayer", tmpl_xrprojection_layer);
    let tmpl_xrquad_layer = super::webxr::create_xrquad_layer_template(scope, templates.get("XRCompositionLayer").copied());
    templates.insert("XRQuadLayer", tmpl_xrquad_layer);
    let tmpl_xrbounded_reference_space = super::webxr::create_xrbounded_reference_space_template(scope, templates.get("XRReferenceSpace").copied());
    templates.insert("XRBoundedReferenceSpace", tmpl_xrbounded_reference_space);
    let tmpl_cdatasection = super::web_apis::create_cdatasection_template(scope, templates.get("Text").copied());
    templates.insert("CDATASection", tmpl_cdatasection);
    let tmpl_svguse_element_shadow_root = super::svg::create_svguse_element_shadow_root_template(scope, templates.get("ShadowRoot").copied());
    templates.insert("SVGUseElementShadowRoot", tmpl_svguse_element_shadow_root);
    let tmpl_htmlanchor_element = super::html_elements::create_htmlanchor_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLAnchorElement", tmpl_htmlanchor_element);
    let tmpl_htmlarea_element = super::html_elements::create_htmlarea_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLAreaElement", tmpl_htmlarea_element);
    let tmpl_htmlbrelement = super::html_elements::create_htmlbrelement_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLBRElement", tmpl_htmlbrelement);
    let tmpl_htmlbase_element = super::html_elements::create_htmlbase_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLBaseElement", tmpl_htmlbase_element);
    let tmpl_htmlbody_element = super::html_elements::create_htmlbody_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLBodyElement", tmpl_htmlbody_element);
    let tmpl_htmlbutton_element = super::html_elements::create_htmlbutton_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLButtonElement", tmpl_htmlbutton_element);
    let tmpl_htmlcanvas_element = super::html_elements::create_htmlcanvas_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLCanvasElement", tmpl_htmlcanvas_element);
    let tmpl_htmldlist_element = super::html_elements::create_htmldlist_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLDListElement", tmpl_htmldlist_element);
    let tmpl_htmldata_element = super::html_elements::create_htmldata_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLDataElement", tmpl_htmldata_element);
    let tmpl_htmldata_list_element = super::html_elements::create_htmldata_list_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLDataListElement", tmpl_htmldata_list_element);
    let tmpl_htmldetails_element = super::html_elements::create_htmldetails_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLDetailsElement", tmpl_htmldetails_element);
    let tmpl_htmldialog_element = super::html_elements::create_htmldialog_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLDialogElement", tmpl_htmldialog_element);
    let tmpl_htmldirectory_element = super::html_elements::create_htmldirectory_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLDirectoryElement", tmpl_htmldirectory_element);
    let tmpl_htmldiv_element = super::html_elements::create_htmldiv_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLDivElement", tmpl_htmldiv_element);
    let tmpl_htmlembed_element = super::html_elements::create_htmlembed_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLEmbedElement", tmpl_htmlembed_element);
    let tmpl_htmlfenced_frame_element = super::html_elements::create_htmlfenced_frame_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLFencedFrameElement", tmpl_htmlfenced_frame_element);
    let tmpl_htmlfield_set_element = super::html_elements::create_htmlfield_set_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLFieldSetElement", tmpl_htmlfield_set_element);
    let tmpl_htmlfont_element = super::html_elements::create_htmlfont_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLFontElement", tmpl_htmlfont_element);
    let tmpl_htmlform_element = super::html_elements::create_htmlform_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLFormElement", tmpl_htmlform_element);
    let tmpl_htmlframe_element = super::html_elements::create_htmlframe_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLFrameElement", tmpl_htmlframe_element);
    let tmpl_htmlframe_set_element = super::html_elements::create_htmlframe_set_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLFrameSetElement", tmpl_htmlframe_set_element);
    let tmpl_htmlgeolocation_element = super::html_elements::create_htmlgeolocation_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLGeolocationElement", tmpl_htmlgeolocation_element);
    let tmpl_htmlhrelement = super::html_elements::create_htmlhrelement_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLHRElement", tmpl_htmlhrelement);
    let tmpl_htmlhead_element = super::html_elements::create_htmlhead_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLHeadElement", tmpl_htmlhead_element);
    let tmpl_htmlheading_element = super::html_elements::create_htmlheading_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLHeadingElement", tmpl_htmlheading_element);
    let tmpl_htmlhtml_element = super::html_elements::create_htmlhtml_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLHtmlElement", tmpl_htmlhtml_element);
    let tmpl_htmliframe_element = super::html_elements::create_htmliframe_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLIFrameElement", tmpl_htmliframe_element);
    let tmpl_htmlimage_element = super::html_elements::create_htmlimage_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLImageElement", tmpl_htmlimage_element);
    let tmpl_htmlinput_element = super::html_elements::create_htmlinput_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLInputElement", tmpl_htmlinput_element);
    let tmpl_htmllielement = super::html_elements::create_htmllielement_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLLIElement", tmpl_htmllielement);
    let tmpl_htmllabel_element = super::html_elements::create_htmllabel_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLLabelElement", tmpl_htmllabel_element);
    let tmpl_htmllegend_element = super::html_elements::create_htmllegend_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLLegendElement", tmpl_htmllegend_element);
    let tmpl_htmllink_element = super::html_elements::create_htmllink_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLLinkElement", tmpl_htmllink_element);
    let tmpl_htmlmap_element = super::html_elements::create_htmlmap_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLMapElement", tmpl_htmlmap_element);
    let tmpl_htmlmarquee_element = super::html_elements::create_htmlmarquee_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLMarqueeElement", tmpl_htmlmarquee_element);
    let tmpl_htmlmedia_element = super::html_elements::create_htmlmedia_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLMediaElement", tmpl_htmlmedia_element);
    let tmpl_htmlmenu_element = super::html_elements::create_htmlmenu_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLMenuElement", tmpl_htmlmenu_element);
    let tmpl_htmlmeta_element = super::html_elements::create_htmlmeta_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLMetaElement", tmpl_htmlmeta_element);
    let tmpl_htmlmeter_element = super::html_elements::create_htmlmeter_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLMeterElement", tmpl_htmlmeter_element);
    let tmpl_htmlmod_element = super::html_elements::create_htmlmod_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLModElement", tmpl_htmlmod_element);
    let tmpl_htmlmodel_element = super::html_elements::create_htmlmodel_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLModelElement", tmpl_htmlmodel_element);
    let tmpl_htmlolist_element = super::html_elements::create_htmlolist_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLOListElement", tmpl_htmlolist_element);
    let tmpl_htmlobject_element = super::html_elements::create_htmlobject_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLObjectElement", tmpl_htmlobject_element);
    let tmpl_htmlopt_group_element = super::html_elements::create_htmlopt_group_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLOptGroupElement", tmpl_htmlopt_group_element);
    let tmpl_htmloption_element = super::html_elements::create_htmloption_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLOptionElement", tmpl_htmloption_element);
    let tmpl_htmloutput_element = super::html_elements::create_htmloutput_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLOutputElement", tmpl_htmloutput_element);
    let tmpl_htmlparagraph_element = super::html_elements::create_htmlparagraph_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLParagraphElement", tmpl_htmlparagraph_element);
    let tmpl_htmlparam_element = super::html_elements::create_htmlparam_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLParamElement", tmpl_htmlparam_element);
    let tmpl_htmlpicture_element = super::html_elements::create_htmlpicture_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLPictureElement", tmpl_htmlpicture_element);
    let tmpl_htmlportal_element = super::html_elements::create_htmlportal_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLPortalElement", tmpl_htmlportal_element);
    let tmpl_htmlpre_element = super::html_elements::create_htmlpre_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLPreElement", tmpl_htmlpre_element);
    let tmpl_htmlprogress_element = super::html_elements::create_htmlprogress_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLProgressElement", tmpl_htmlprogress_element);
    let tmpl_htmlquote_element = super::html_elements::create_htmlquote_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLQuoteElement", tmpl_htmlquote_element);
    let tmpl_htmlscript_element = super::html_elements::create_htmlscript_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLScriptElement", tmpl_htmlscript_element);
    let tmpl_htmlselect_element = super::html_elements::create_htmlselect_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLSelectElement", tmpl_htmlselect_element);
    let tmpl_htmlselected_content_element = super::html_elements::create_htmlselected_content_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLSelectedContentElement", tmpl_htmlselected_content_element);
    let tmpl_htmlslot_element = super::html_elements::create_htmlslot_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLSlotElement", tmpl_htmlslot_element);
    let tmpl_htmlsource_element = super::html_elements::create_htmlsource_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLSourceElement", tmpl_htmlsource_element);
    let tmpl_htmlspan_element = super::html_elements::create_htmlspan_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLSpanElement", tmpl_htmlspan_element);
    let tmpl_htmlstyle_element = super::html_elements::create_htmlstyle_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLStyleElement", tmpl_htmlstyle_element);
    let tmpl_htmltable_caption_element = super::html_elements::create_htmltable_caption_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLTableCaptionElement", tmpl_htmltable_caption_element);
    let tmpl_htmltable_cell_element = super::html_elements::create_htmltable_cell_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLTableCellElement", tmpl_htmltable_cell_element);
    let tmpl_htmltable_col_element = super::html_elements::create_htmltable_col_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLTableColElement", tmpl_htmltable_col_element);
    let tmpl_htmltable_element = super::html_elements::create_htmltable_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLTableElement", tmpl_htmltable_element);
    let tmpl_htmltable_row_element = super::html_elements::create_htmltable_row_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLTableRowElement", tmpl_htmltable_row_element);
    let tmpl_htmltable_section_element = super::html_elements::create_htmltable_section_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLTableSectionElement", tmpl_htmltable_section_element);
    let tmpl_htmltemplate_element = super::html_elements::create_htmltemplate_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLTemplateElement", tmpl_htmltemplate_element);
    let tmpl_htmltext_area_element = super::html_elements::create_htmltext_area_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLTextAreaElement", tmpl_htmltext_area_element);
    let tmpl_htmltime_element = super::html_elements::create_htmltime_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLTimeElement", tmpl_htmltime_element);
    let tmpl_htmltitle_element = super::html_elements::create_htmltitle_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLTitleElement", tmpl_htmltitle_element);
    let tmpl_htmltrack_element = super::html_elements::create_htmltrack_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLTrackElement", tmpl_htmltrack_element);
    let tmpl_htmlulist_element = super::html_elements::create_htmlulist_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLUListElement", tmpl_htmlulist_element);
    let tmpl_htmlunknown_element = super::html_elements::create_htmlunknown_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLUnknownElement", tmpl_htmlunknown_element);
    let tmpl_math_mlanchor_element = super::web_apis::create_math_mlanchor_element_template(scope, templates.get("MathMLElement").copied());
    templates.insert("MathMLAnchorElement", tmpl_math_mlanchor_element);
    let tmpl_svganimation_element = super::svg::create_svganimation_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGAnimationElement", tmpl_svganimation_element);
    let tmpl_svgclip_path_element = super::svg::create_svgclip_path_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGClipPathElement", tmpl_svgclip_path_element);
    let tmpl_svgcomponent_transfer_function_element = super::svg::create_svgcomponent_transfer_function_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGComponentTransferFunctionElement", tmpl_svgcomponent_transfer_function_element);
    let tmpl_svgdesc_element = super::svg::create_svgdesc_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGDescElement", tmpl_svgdesc_element);
    let tmpl_svgfeblend_element = super::svg::create_svgfeblend_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEBlendElement", tmpl_svgfeblend_element);
    let tmpl_svgfecolor_matrix_element = super::svg::create_svgfecolor_matrix_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEColorMatrixElement", tmpl_svgfecolor_matrix_element);
    let tmpl_svgfecomponent_transfer_element = super::svg::create_svgfecomponent_transfer_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEComponentTransferElement", tmpl_svgfecomponent_transfer_element);
    let tmpl_svgfecomposite_element = super::svg::create_svgfecomposite_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFECompositeElement", tmpl_svgfecomposite_element);
    let tmpl_svgfeconvolve_matrix_element = super::svg::create_svgfeconvolve_matrix_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEConvolveMatrixElement", tmpl_svgfeconvolve_matrix_element);
    let tmpl_svgfediffuse_lighting_element = super::svg::create_svgfediffuse_lighting_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEDiffuseLightingElement", tmpl_svgfediffuse_lighting_element);
    let tmpl_svgfedisplacement_map_element = super::svg::create_svgfedisplacement_map_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEDisplacementMapElement", tmpl_svgfedisplacement_map_element);
    let tmpl_svgfedistant_light_element = super::svg::create_svgfedistant_light_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEDistantLightElement", tmpl_svgfedistant_light_element);
    let tmpl_svgfedrop_shadow_element = super::svg::create_svgfedrop_shadow_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEDropShadowElement", tmpl_svgfedrop_shadow_element);
    let tmpl_svgfeflood_element = super::svg::create_svgfeflood_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEFloodElement", tmpl_svgfeflood_element);
    let tmpl_svgfegaussian_blur_element = super::svg::create_svgfegaussian_blur_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEGaussianBlurElement", tmpl_svgfegaussian_blur_element);
    let tmpl_svgfeimage_element = super::svg::create_svgfeimage_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEImageElement", tmpl_svgfeimage_element);
    let tmpl_svgfemerge_element = super::svg::create_svgfemerge_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEMergeElement", tmpl_svgfemerge_element);
    let tmpl_svgfemerge_node_element = super::svg::create_svgfemerge_node_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEMergeNodeElement", tmpl_svgfemerge_node_element);
    let tmpl_svgfemorphology_element = super::svg::create_svgfemorphology_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEMorphologyElement", tmpl_svgfemorphology_element);
    let tmpl_svgfeoffset_element = super::svg::create_svgfeoffset_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEOffsetElement", tmpl_svgfeoffset_element);
    let tmpl_svgfepoint_light_element = super::svg::create_svgfepoint_light_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEPointLightElement", tmpl_svgfepoint_light_element);
    let tmpl_svgfespecular_lighting_element = super::svg::create_svgfespecular_lighting_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFESpecularLightingElement", tmpl_svgfespecular_lighting_element);
    let tmpl_svgfespot_light_element = super::svg::create_svgfespot_light_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFESpotLightElement", tmpl_svgfespot_light_element);
    let tmpl_svgfetile_element = super::svg::create_svgfetile_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFETileElement", tmpl_svgfetile_element);
    let tmpl_svgfeturbulence_element = super::svg::create_svgfeturbulence_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFETurbulenceElement", tmpl_svgfeturbulence_element);
    let tmpl_svgfilter_element = super::svg::create_svgfilter_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFilterElement", tmpl_svgfilter_element);
    let tmpl_svggradient_element = super::svg::create_svggradient_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGGradientElement", tmpl_svggradient_element);
    let tmpl_svggraphics_element = super::svg::create_svggraphics_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGGraphicsElement", tmpl_svggraphics_element);
    let tmpl_svgmpath_element = super::svg::create_svgmpath_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGMPathElement", tmpl_svgmpath_element);
    let tmpl_svgmarker_element = super::svg::create_svgmarker_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGMarkerElement", tmpl_svgmarker_element);
    let tmpl_svgmask_element = super::svg::create_svgmask_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGMaskElement", tmpl_svgmask_element);
    let tmpl_svgmetadata_element = super::svg::create_svgmetadata_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGMetadataElement", tmpl_svgmetadata_element);
    let tmpl_svgpattern_element = super::svg::create_svgpattern_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGPatternElement", tmpl_svgpattern_element);
    let tmpl_svgscript_element = super::svg::create_svgscript_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGScriptElement", tmpl_svgscript_element);
    let tmpl_svgstop_element = super::svg::create_svgstop_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGStopElement", tmpl_svgstop_element);
    let tmpl_svgstyle_element = super::svg::create_svgstyle_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGStyleElement", tmpl_svgstyle_element);
    let tmpl_svgtitle_element = super::svg::create_svgtitle_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGTitleElement", tmpl_svgtitle_element);
    let tmpl_svgview_element = super::svg::create_svgview_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGViewElement", tmpl_svgview_element);
    let tmpl_htmlaudio_element = super::html_elements::create_htmlaudio_element_template(scope, templates.get("HTMLMediaElement").copied());
    templates.insert("HTMLAudioElement", tmpl_htmlaudio_element);
    let tmpl_htmlvideo_element = super::html_elements::create_htmlvideo_element_template(scope, templates.get("HTMLMediaElement").copied());
    templates.insert("HTMLVideoElement", tmpl_htmlvideo_element);
    let tmpl_svganimate_element = super::svg::create_svganimate_element_template(scope, templates.get("SVGAnimationElement").copied());
    templates.insert("SVGAnimateElement", tmpl_svganimate_element);
    let tmpl_svganimate_motion_element = super::svg::create_svganimate_motion_element_template(scope, templates.get("SVGAnimationElement").copied());
    templates.insert("SVGAnimateMotionElement", tmpl_svganimate_motion_element);
    let tmpl_svganimate_transform_element = super::svg::create_svganimate_transform_element_template(scope, templates.get("SVGAnimationElement").copied());
    templates.insert("SVGAnimateTransformElement", tmpl_svganimate_transform_element);
    let tmpl_svgset_element = super::svg::create_svgset_element_template(scope, templates.get("SVGAnimationElement").copied());
    templates.insert("SVGSetElement", tmpl_svgset_element);
    let tmpl_svgfefunc_aelement = super::svg::create_svgfefunc_aelement_template(scope, templates.get("SVGComponentTransferFunctionElement").copied());
    templates.insert("SVGFEFuncAElement", tmpl_svgfefunc_aelement);
    let tmpl_svgfefunc_belement = super::svg::create_svgfefunc_belement_template(scope, templates.get("SVGComponentTransferFunctionElement").copied());
    templates.insert("SVGFEFuncBElement", tmpl_svgfefunc_belement);
    let tmpl_svgfefunc_gelement = super::svg::create_svgfefunc_gelement_template(scope, templates.get("SVGComponentTransferFunctionElement").copied());
    templates.insert("SVGFEFuncGElement", tmpl_svgfefunc_gelement);
    let tmpl_svgfefunc_relement = super::svg::create_svgfefunc_relement_template(scope, templates.get("SVGComponentTransferFunctionElement").copied());
    templates.insert("SVGFEFuncRElement", tmpl_svgfefunc_relement);
    let tmpl_svglinear_gradient_element = super::svg::create_svglinear_gradient_element_template(scope, templates.get("SVGGradientElement").copied());
    templates.insert("SVGLinearGradientElement", tmpl_svglinear_gradient_element);
    let tmpl_svgradial_gradient_element = super::svg::create_svgradial_gradient_element_template(scope, templates.get("SVGGradientElement").copied());
    templates.insert("SVGRadialGradientElement", tmpl_svgradial_gradient_element);
    let tmpl_svgaelement = super::svg::create_svgaelement_template(scope, templates.get("SVGGraphicsElement").copied());
    templates.insert("SVGAElement", tmpl_svgaelement);
    let tmpl_svgdefs_element = super::svg::create_svgdefs_element_template(scope, templates.get("SVGGraphicsElement").copied());
    templates.insert("SVGDefsElement", tmpl_svgdefs_element);
    let tmpl_svgforeign_object_element = super::svg::create_svgforeign_object_element_template(scope, templates.get("SVGGraphicsElement").copied());
    templates.insert("SVGForeignObjectElement", tmpl_svgforeign_object_element);
    let tmpl_svggelement = super::svg::create_svggelement_template(scope, templates.get("SVGGraphicsElement").copied());
    templates.insert("SVGGElement", tmpl_svggelement);
    let tmpl_svggeometry_element = super::svg::create_svggeometry_element_template(scope, templates.get("SVGGraphicsElement").copied());
    templates.insert("SVGGeometryElement", tmpl_svggeometry_element);
    let tmpl_svgimage_element = super::svg::create_svgimage_element_template(scope, templates.get("SVGGraphicsElement").copied());
    templates.insert("SVGImageElement", tmpl_svgimage_element);
    let tmpl_svgsvgelement = super::svg::create_svgsvgelement_template(scope, templates.get("SVGGraphicsElement").copied());
    templates.insert("SVGSVGElement", tmpl_svgsvgelement);
    let tmpl_svgswitch_element = super::svg::create_svgswitch_element_template(scope, templates.get("SVGGraphicsElement").copied());
    templates.insert("SVGSwitchElement", tmpl_svgswitch_element);
    let tmpl_svgsymbol_element = super::svg::create_svgsymbol_element_template(scope, templates.get("SVGGraphicsElement").copied());
    templates.insert("SVGSymbolElement", tmpl_svgsymbol_element);
    let tmpl_svgtext_content_element = super::svg::create_svgtext_content_element_template(scope, templates.get("SVGGraphicsElement").copied());
    templates.insert("SVGTextContentElement", tmpl_svgtext_content_element);
    let tmpl_svguse_element = super::svg::create_svguse_element_template(scope, templates.get("SVGGraphicsElement").copied());
    templates.insert("SVGUseElement", tmpl_svguse_element);
    let tmpl_svgcircle_element = super::svg::create_svgcircle_element_template(scope, templates.get("SVGGeometryElement").copied());
    templates.insert("SVGCircleElement", tmpl_svgcircle_element);
    let tmpl_svgellipse_element = super::svg::create_svgellipse_element_template(scope, templates.get("SVGGeometryElement").copied());
    templates.insert("SVGEllipseElement", tmpl_svgellipse_element);
    let tmpl_svgline_element = super::svg::create_svgline_element_template(scope, templates.get("SVGGeometryElement").copied());
    templates.insert("SVGLineElement", tmpl_svgline_element);
    let tmpl_svgpath_element = super::svg::create_svgpath_element_template(scope, templates.get("SVGGeometryElement").copied());
    templates.insert("SVGPathElement", tmpl_svgpath_element);
    let tmpl_svgpolygon_element = super::svg::create_svgpolygon_element_template(scope, templates.get("SVGGeometryElement").copied());
    templates.insert("SVGPolygonElement", tmpl_svgpolygon_element);
    let tmpl_svgpolyline_element = super::svg::create_svgpolyline_element_template(scope, templates.get("SVGGeometryElement").copied());
    templates.insert("SVGPolylineElement", tmpl_svgpolyline_element);
    let tmpl_svgrect_element = super::svg::create_svgrect_element_template(scope, templates.get("SVGGeometryElement").copied());
    templates.insert("SVGRectElement", tmpl_svgrect_element);
    let tmpl_svgtext_path_element = super::svg::create_svgtext_path_element_template(scope, templates.get("SVGTextContentElement").copied());
    templates.insert("SVGTextPathElement", tmpl_svgtext_path_element);
    let tmpl_svgtext_positioning_element = super::svg::create_svgtext_positioning_element_template(scope, templates.get("SVGTextContentElement").copied());
    templates.insert("SVGTextPositioningElement", tmpl_svgtext_positioning_element);
    let tmpl_svgtspan_element = super::svg::create_svgtspan_element_template(scope, templates.get("SVGTextPositioningElement").copied());
    templates.insert("SVGTSpanElement", tmpl_svgtspan_element);
    let tmpl_svgtext_element = super::svg::create_svgtext_element_template(scope, templates.get("SVGTextPositioningElement").copied());
    templates.insert("SVGTextElement", tmpl_svgtext_element);

    // Register constructors on global (non-enumerable)
    // ANGLE_instanced_arrays: NoInterfaceObject — skip global registration
    if let Some(ctor_abort_controller) = tmpl_abort_controller.get_function(scope) {
        let name_abort_controller = v8::String::new(scope, "AbortController").unwrap();
        global.define_own_property(scope, name_abort_controller.into(), ctor_abort_controller.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_abstract_range) = tmpl_abstract_range.get_function(scope) {
        let name_abstract_range = v8::String::new(scope, "AbstractRange").unwrap();
        global.define_own_property(scope, name_abstract_range.into(), ctor_abstract_range.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_animation_effect) = tmpl_animation_effect.get_function(scope) {
        let name_animation_effect = v8::String::new(scope, "AnimationEffect").unwrap();
        global.define_own_property(scope, name_animation_effect.into(), ctor_animation_effect.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_animation_node_list) = tmpl_animation_node_list.get_function(scope) {
        let name_animation_node_list = v8::String::new(scope, "AnimationNodeList").unwrap();
        global.define_own_property(scope, name_animation_node_list.into(), ctor_animation_node_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_animation_timeline) = tmpl_animation_timeline.get_function(scope) {
        let name_animation_timeline = v8::String::new(scope, "AnimationTimeline").unwrap();
        global.define_own_property(scope, name_animation_timeline.into(), ctor_animation_timeline.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_animation_trigger) = tmpl_animation_trigger.get_function(scope) {
        let name_animation_trigger = v8::String::new(scope, "AnimationTrigger").unwrap();
        global.define_own_property(scope, name_animation_trigger.into(), ctor_animation_trigger.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_attribution) = tmpl_attribution.get_function(scope) {
        let name_attribution = v8::String::new(scope, "Attribution").unwrap();
        global.define_own_property(scope, name_attribution.into(), ctor_attribution.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_attribution_aggregation_services) = tmpl_attribution_aggregation_services.get_function(scope) {
        let name_attribution_aggregation_services = v8::String::new(scope, "AttributionAggregationServices").unwrap();
        global.define_own_property(scope, name_attribution_aggregation_services.into(), ctor_attribution_aggregation_services.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_audio_buffer) = tmpl_audio_buffer.get_function(scope) {
        let name_audio_buffer = v8::String::new(scope, "AudioBuffer").unwrap();
        global.define_own_property(scope, name_audio_buffer.into(), ctor_audio_buffer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_audio_data) = tmpl_audio_data.get_function(scope) {
        let name_audio_data = v8::String::new(scope, "AudioData").unwrap();
        global.define_own_property(scope, name_audio_data.into(), ctor_audio_data.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_audio_listener) = tmpl_audio_listener.get_function(scope) {
        let name_audio_listener = v8::String::new(scope, "AudioListener").unwrap();
        global.define_own_property(scope, name_audio_listener.into(), ctor_audio_listener.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_audio_param) = tmpl_audio_param.get_function(scope) {
        let name_audio_param = v8::String::new(scope, "AudioParam").unwrap();
        global.define_own_property(scope, name_audio_param.into(), ctor_audio_param.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_audio_param_map) = tmpl_audio_param_map.get_function(scope) {
        let name_audio_param_map = v8::String::new(scope, "AudioParamMap").unwrap();
        global.define_own_property(scope, name_audio_param_map.into(), ctor_audio_param_map.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_audio_playback_stats) = tmpl_audio_playback_stats.get_function(scope) {
        let name_audio_playback_stats = v8::String::new(scope, "AudioPlaybackStats").unwrap();
        global.define_own_property(scope, name_audio_playback_stats.into(), ctor_audio_playback_stats.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_audio_sink_info) = tmpl_audio_sink_info.get_function(scope) {
        let name_audio_sink_info = v8::String::new(scope, "AudioSinkInfo").unwrap();
        global.define_own_property(scope, name_audio_sink_info.into(), ctor_audio_sink_info.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_audio_track) = tmpl_audio_track.get_function(scope) {
        let name_audio_track = v8::String::new(scope, "AudioTrack").unwrap();
        global.define_own_property(scope, name_audio_track.into(), ctor_audio_track.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_audio_worklet_processor) = tmpl_audio_worklet_processor.get_function(scope) {
        let name_audio_worklet_processor = v8::String::new(scope, "AudioWorkletProcessor").unwrap();
        global.define_own_property(scope, name_audio_worklet_processor.into(), ctor_audio_worklet_processor.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_authenticator_response) = tmpl_authenticator_response.get_function(scope) {
        let name_authenticator_response = v8::String::new(scope, "AuthenticatorResponse").unwrap();
        global.define_own_property(scope, name_authenticator_response.into(), ctor_authenticator_response.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_background_fetch_manager) = tmpl_background_fetch_manager.get_function(scope) {
        let name_background_fetch_manager = v8::String::new(scope, "BackgroundFetchManager").unwrap();
        global.define_own_property(scope, name_background_fetch_manager.into(), ctor_background_fetch_manager.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_background_fetch_record) = tmpl_background_fetch_record.get_function(scope) {
        let name_background_fetch_record = v8::String::new(scope, "BackgroundFetchRecord").unwrap();
        global.define_own_property(scope, name_background_fetch_record.into(), ctor_background_fetch_record.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bar_prop) = tmpl_bar_prop.get_function(scope) {
        let name_bar_prop = v8::String::new(scope, "BarProp").unwrap();
        global.define_own_property(scope, name_bar_prop.into(), ctor_bar_prop.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_barcode_detector) = tmpl_barcode_detector.get_function(scope) {
        let name_barcode_detector = v8::String::new(scope, "BarcodeDetector").unwrap();
        global.define_own_property(scope, name_barcode_detector.into(), ctor_barcode_detector.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_baseline) = tmpl_baseline.get_function(scope) {
        let name_baseline = v8::String::new(scope, "Baseline").unwrap();
        global.define_own_property(scope, name_baseline.into(), ctor_baseline.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_blob) = tmpl_blob.get_function(scope) {
        let name_blob = v8::String::new(scope, "Blob").unwrap();
        global.define_own_property(scope, name_blob.into(), ctor_blob.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_characteristic_properties) = tmpl_bluetooth_characteristic_properties.get_function(scope) {
        let name_bluetooth_characteristic_properties = v8::String::new(scope, "BluetoothCharacteristicProperties").unwrap();
        global.define_own_property(scope, name_bluetooth_characteristic_properties.into(), ctor_bluetooth_characteristic_properties.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_data_filter) = tmpl_bluetooth_data_filter.get_function(scope) {
        let name_bluetooth_data_filter = v8::String::new(scope, "BluetoothDataFilter").unwrap();
        global.define_own_property(scope, name_bluetooth_data_filter.into(), ctor_bluetooth_data_filter.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_lescan) = tmpl_bluetooth_lescan.get_function(scope) {
        let name_bluetooth_lescan = v8::String::new(scope, "BluetoothLEScan").unwrap();
        global.define_own_property(scope, name_bluetooth_lescan.into(), ctor_bluetooth_lescan.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_lescan_filter) = tmpl_bluetooth_lescan_filter.get_function(scope) {
        let name_bluetooth_lescan_filter = v8::String::new(scope, "BluetoothLEScanFilter").unwrap();
        global.define_own_property(scope, name_bluetooth_lescan_filter.into(), ctor_bluetooth_lescan_filter.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_manufacturer_data_filter) = tmpl_bluetooth_manufacturer_data_filter.get_function(scope) {
        let name_bluetooth_manufacturer_data_filter = v8::String::new(scope, "BluetoothManufacturerDataFilter").unwrap();
        global.define_own_property(scope, name_bluetooth_manufacturer_data_filter.into(), ctor_bluetooth_manufacturer_data_filter.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_manufacturer_data_map) = tmpl_bluetooth_manufacturer_data_map.get_function(scope) {
        let name_bluetooth_manufacturer_data_map = v8::String::new(scope, "BluetoothManufacturerDataMap").unwrap();
        global.define_own_property(scope, name_bluetooth_manufacturer_data_map.into(), ctor_bluetooth_manufacturer_data_map.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_remote_gattdescriptor) = tmpl_bluetooth_remote_gattdescriptor.get_function(scope) {
        let name_bluetooth_remote_gattdescriptor = v8::String::new(scope, "BluetoothRemoteGATTDescriptor").unwrap();
        global.define_own_property(scope, name_bluetooth_remote_gattdescriptor.into(), ctor_bluetooth_remote_gattdescriptor.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_remote_gattserver) = tmpl_bluetooth_remote_gattserver.get_function(scope) {
        let name_bluetooth_remote_gattserver = v8::String::new(scope, "BluetoothRemoteGATTServer").unwrap();
        global.define_own_property(scope, name_bluetooth_remote_gattserver.into(), ctor_bluetooth_remote_gattserver.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_service_data_filter) = tmpl_bluetooth_service_data_filter.get_function(scope) {
        let name_bluetooth_service_data_filter = v8::String::new(scope, "BluetoothServiceDataFilter").unwrap();
        global.define_own_property(scope, name_bluetooth_service_data_filter.into(), ctor_bluetooth_service_data_filter.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_service_data_map) = tmpl_bluetooth_service_data_map.get_function(scope) {
        let name_bluetooth_service_data_map = v8::String::new(scope, "BluetoothServiceDataMap").unwrap();
        global.define_own_property(scope, name_bluetooth_service_data_map.into(), ctor_bluetooth_service_data_map.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_uuid) = tmpl_bluetooth_uuid.get_function(scope) {
        let name_bluetooth_uuid = v8::String::new(scope, "BluetoothUUID").unwrap();
        global.define_own_property(scope, name_bluetooth_uuid.into(), ctor_bluetooth_uuid.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_break_token) = tmpl_break_token.get_function(scope) {
        let name_break_token = v8::String::new(scope, "BreakToken").unwrap();
        global.define_own_property(scope, name_break_token.into(), ctor_break_token.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_byte_length_queuing_strategy) = tmpl_byte_length_queuing_strategy.get_function(scope) {
        let name_byte_length_queuing_strategy = v8::String::new(scope, "ByteLengthQueuingStrategy").unwrap();
        global.define_own_property(scope, name_byte_length_queuing_strategy.into(), ctor_byte_length_queuing_strategy.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssfont_feature_values_map) = tmpl_cssfont_feature_values_map.get_function(scope) {
        let name_cssfont_feature_values_map = v8::String::new(scope, "CSSFontFeatureValuesMap").unwrap();
        global.define_own_property(scope, name_cssfont_feature_values_map.into(), ctor_cssfont_feature_values_map.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssnumeric_array) = tmpl_cssnumeric_array.get_function(scope) {
        let name_cssnumeric_array = v8::String::new(scope, "CSSNumericArray").unwrap();
        global.define_own_property(scope, name_cssnumeric_array.into(), ctor_cssnumeric_array.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssparser_rule) = tmpl_cssparser_rule.get_function(scope) {
        let name_cssparser_rule = v8::String::new(scope, "CSSParserRule").unwrap();
        global.define_own_property(scope, name_cssparser_rule.into(), ctor_cssparser_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssparser_value) = tmpl_cssparser_value.get_function(scope) {
        let name_cssparser_value = v8::String::new(scope, "CSSParserValue").unwrap();
        global.define_own_property(scope, name_cssparser_value.into(), ctor_cssparser_value.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csspseudo_element) = tmpl_csspseudo_element.get_function(scope) {
        let name_csspseudo_element = v8::String::new(scope, "CSSPseudoElement").unwrap();
        global.define_own_property(scope, name_csspseudo_element.into(), ctor_csspseudo_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssrule) = tmpl_cssrule.get_function(scope) {
        let name_cssrule = v8::String::new(scope, "CSSRule").unwrap();
        global.define_own_property(scope, name_cssrule.into(), ctor_cssrule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssrule_list) = tmpl_cssrule_list.get_function(scope) {
        let name_cssrule_list = v8::String::new(scope, "CSSRuleList").unwrap();
        global.define_own_property(scope, name_cssrule_list.into(), ctor_cssrule_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssstyle_declaration) = tmpl_cssstyle_declaration.get_function(scope) {
        let name_cssstyle_declaration = v8::String::new(scope, "CSSStyleDeclaration").unwrap();
        global.define_own_property(scope, name_cssstyle_declaration.into(), ctor_cssstyle_declaration.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssstyle_value) = tmpl_cssstyle_value.get_function(scope) {
        let name_cssstyle_value = v8::String::new(scope, "CSSStyleValue").unwrap();
        global.define_own_property(scope, name_cssstyle_value.into(), ctor_cssstyle_value.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csstransform_component) = tmpl_csstransform_component.get_function(scope) {
        let name_csstransform_component = v8::String::new(scope, "CSSTransformComponent").unwrap();
        global.define_own_property(scope, name_csstransform_component.into(), ctor_csstransform_component.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssvariable_reference_value) = tmpl_cssvariable_reference_value.get_function(scope) {
        let name_cssvariable_reference_value = v8::String::new(scope, "CSSVariableReferenceValue").unwrap();
        global.define_own_property(scope, name_cssvariable_reference_value.into(), ctor_cssvariable_reference_value.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cache) = tmpl_cache.get_function(scope) {
        let name_cache = v8::String::new(scope, "Cache").unwrap();
        global.define_own_property(scope, name_cache.into(), ctor_cache.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cache_storage) = tmpl_cache_storage.get_function(scope) {
        let name_cache_storage = v8::String::new(scope, "CacheStorage").unwrap();
        global.define_own_property(scope, name_cache_storage.into(), ctor_cache_storage.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_canvas_gradient) = tmpl_canvas_gradient.get_function(scope) {
        let name_canvas_gradient = v8::String::new(scope, "CanvasGradient").unwrap();
        global.define_own_property(scope, name_canvas_gradient.into(), ctor_canvas_gradient.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_canvas_pattern) = tmpl_canvas_pattern.get_function(scope) {
        let name_canvas_pattern = v8::String::new(scope, "CanvasPattern").unwrap();
        global.define_own_property(scope, name_canvas_pattern.into(), ctor_canvas_pattern.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_canvas_rendering_context2_d) = tmpl_canvas_rendering_context2_d.get_function(scope) {
        let name_canvas_rendering_context2_d = v8::String::new(scope, "CanvasRenderingContext2D").unwrap();
        global.define_own_property(scope, name_canvas_rendering_context2_d.into(), ctor_canvas_rendering_context2_d.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_caret_position) = tmpl_caret_position.get_function(scope) {
        let name_caret_position = v8::String::new(scope, "CaretPosition").unwrap();
        global.define_own_property(scope, name_caret_position.into(), ctor_caret_position.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chapter_information) = tmpl_chapter_information.get_function(scope) {
        let name_chapter_information = v8::String::new(scope, "ChapterInformation").unwrap();
        global.define_own_property(scope, name_chapter_information.into(), ctor_chapter_information.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_child_break_token) = tmpl_child_break_token.get_function(scope) {
        let name_child_break_token = v8::String::new(scope, "ChildBreakToken").unwrap();
        global.define_own_property(scope, name_child_break_token.into(), ctor_child_break_token.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_accessibility_features) = tmpl_chrome_accessibility_features.get_function(scope) {
        let name_chrome_accessibility_features = v8::String::new(scope, "ChromeAccessibilityFeatures").unwrap();
        global.define_own_property(scope, name_chrome_accessibility_features.into(), ctor_chrome_accessibility_features.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_action) = tmpl_chrome_action.get_function(scope) {
        let name_chrome_action = v8::String::new(scope, "ChromeAction").unwrap();
        global.define_own_property(scope, name_chrome_action.into(), ctor_chrome_action.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_alarms) = tmpl_chrome_alarms.get_function(scope) {
        let name_chrome_alarms = v8::String::new(scope, "ChromeAlarms").unwrap();
        global.define_own_property(scope, name_chrome_alarms.into(), ctor_chrome_alarms.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_app) = tmpl_chrome_app.get_function(scope) {
        let name_chrome_app = v8::String::new(scope, "ChromeApp").unwrap();
        global.define_own_property(scope, name_chrome_app.into(), ctor_chrome_app.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_app_runtime) = tmpl_chrome_app_runtime.get_function(scope) {
        let name_chrome_app_runtime = v8::String::new(scope, "ChromeAppRuntime").unwrap();
        global.define_own_property(scope, name_chrome_app_runtime.into(), ctor_chrome_app_runtime.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_app_window) = tmpl_chrome_app_window.get_function(scope) {
        let name_chrome_app_window = v8::String::new(scope, "ChromeAppWindow").unwrap();
        global.define_own_property(scope, name_chrome_app_window.into(), ctor_chrome_app_window.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_audio) = tmpl_chrome_audio.get_function(scope) {
        let name_chrome_audio = v8::String::new(scope, "ChromeAudio").unwrap();
        global.define_own_property(scope, name_chrome_audio.into(), ctor_chrome_audio.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_autofill_private) = tmpl_chrome_autofill_private.get_function(scope) {
        let name_chrome_autofill_private = v8::String::new(scope, "ChromeAutofillPrivate").unwrap();
        global.define_own_property(scope, name_chrome_autofill_private.into(), ctor_chrome_autofill_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_bluetooth) = tmpl_chrome_bluetooth.get_function(scope) {
        let name_chrome_bluetooth = v8::String::new(scope, "ChromeBluetooth").unwrap();
        global.define_own_property(scope, name_chrome_bluetooth.into(), ctor_chrome_bluetooth.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_bluetooth_low_energy) = tmpl_chrome_bluetooth_low_energy.get_function(scope) {
        let name_chrome_bluetooth_low_energy = v8::String::new(scope, "ChromeBluetoothLowEnergy").unwrap();
        global.define_own_property(scope, name_chrome_bluetooth_low_energy.into(), ctor_chrome_bluetooth_low_energy.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_bluetooth_socket) = tmpl_chrome_bluetooth_socket.get_function(scope) {
        let name_chrome_bluetooth_socket = v8::String::new(scope, "ChromeBluetoothSocket").unwrap();
        global.define_own_property(scope, name_chrome_bluetooth_socket.into(), ctor_chrome_bluetooth_socket.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_bookmarks) = tmpl_chrome_bookmarks.get_function(scope) {
        let name_chrome_bookmarks = v8::String::new(scope, "ChromeBookmarks").unwrap();
        global.define_own_property(scope, name_chrome_bookmarks.into(), ctor_chrome_bookmarks.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_braille_display_private) = tmpl_chrome_braille_display_private.get_function(scope) {
        let name_chrome_braille_display_private = v8::String::new(scope, "ChromeBrailleDisplayPrivate").unwrap();
        global.define_own_property(scope, name_chrome_braille_display_private.into(), ctor_chrome_braille_display_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_browsing_data) = tmpl_chrome_browsing_data.get_function(scope) {
        let name_chrome_browsing_data = v8::String::new(scope, "ChromeBrowsingData").unwrap();
        global.define_own_property(scope, name_chrome_browsing_data.into(), ctor_chrome_browsing_data.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_csi) = tmpl_chrome_csi.get_function(scope) {
        let name_chrome_csi = v8::String::new(scope, "ChromeCSI").unwrap();
        global.define_own_property(scope, name_chrome_csi.into(), ctor_chrome_csi.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_cast) = tmpl_chrome_cast.get_function(scope) {
        let name_chrome_cast = v8::String::new(scope, "ChromeCast").unwrap();
        global.define_own_property(scope, name_chrome_cast.into(), ctor_chrome_cast.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_cast_streaming) = tmpl_chrome_cast_streaming.get_function(scope) {
        let name_chrome_cast_streaming = v8::String::new(scope, "ChromeCastStreaming").unwrap();
        global.define_own_property(scope, name_chrome_cast_streaming.into(), ctor_chrome_cast_streaming.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_certificate_provider) = tmpl_chrome_certificate_provider.get_function(scope) {
        let name_chrome_certificate_provider = v8::String::new(scope, "ChromeCertificateProvider").unwrap();
        global.define_own_property(scope, name_chrome_certificate_provider.into(), ctor_chrome_certificate_provider.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_clipboard) = tmpl_chrome_clipboard.get_function(scope) {
        let name_chrome_clipboard = v8::String::new(scope, "ChromeClipboard").unwrap();
        global.define_own_property(scope, name_chrome_clipboard.into(), ctor_chrome_clipboard.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_command_line_private) = tmpl_chrome_command_line_private.get_function(scope) {
        let name_chrome_command_line_private = v8::String::new(scope, "ChromeCommandLinePrivate").unwrap();
        global.define_own_property(scope, name_chrome_command_line_private.into(), ctor_chrome_command_line_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_commands) = tmpl_chrome_commands.get_function(scope) {
        let name_chrome_commands = v8::String::new(scope, "ChromeCommands").unwrap();
        global.define_own_property(scope, name_chrome_commands.into(), ctor_chrome_commands.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_content_settings) = tmpl_chrome_content_settings.get_function(scope) {
        let name_chrome_content_settings = v8::String::new(scope, "ChromeContentSettings").unwrap();
        global.define_own_property(scope, name_chrome_content_settings.into(), ctor_chrome_content_settings.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_context_menus) = tmpl_chrome_context_menus.get_function(scope) {
        let name_chrome_context_menus = v8::String::new(scope, "ChromeContextMenus").unwrap();
        global.define_own_property(scope, name_chrome_context_menus.into(), ctor_chrome_context_menus.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_cookies) = tmpl_chrome_cookies.get_function(scope) {
        let name_chrome_cookies = v8::String::new(scope, "ChromeCookies").unwrap();
        global.define_own_property(scope, name_chrome_cookies.into(), ctor_chrome_cookies.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_dom) = tmpl_chrome_dom.get_function(scope) {
        let name_chrome_dom = v8::String::new(scope, "ChromeDOM").unwrap();
        global.define_own_property(scope, name_chrome_dom.into(), ctor_chrome_dom.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_debugger) = tmpl_chrome_debugger.get_function(scope) {
        let name_chrome_debugger = v8::String::new(scope, "ChromeDebugger").unwrap();
        global.define_own_property(scope, name_chrome_debugger.into(), ctor_chrome_debugger.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_declarative_content) = tmpl_chrome_declarative_content.get_function(scope) {
        let name_chrome_declarative_content = v8::String::new(scope, "ChromeDeclarativeContent").unwrap();
        global.define_own_property(scope, name_chrome_declarative_content.into(), ctor_chrome_declarative_content.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_declarative_net_request) = tmpl_chrome_declarative_net_request.get_function(scope) {
        let name_chrome_declarative_net_request = v8::String::new(scope, "ChromeDeclarativeNetRequest").unwrap();
        global.define_own_property(scope, name_chrome_declarative_net_request.into(), ctor_chrome_declarative_net_request.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_desk_capture) = tmpl_chrome_desk_capture.get_function(scope) {
        let name_chrome_desk_capture = v8::String::new(scope, "ChromeDeskCapture").unwrap();
        global.define_own_property(scope, name_chrome_desk_capture.into(), ctor_chrome_desk_capture.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_desktop_capture) = tmpl_chrome_desktop_capture.get_function(scope) {
        let name_chrome_desktop_capture = v8::String::new(scope, "ChromeDesktopCapture").unwrap();
        global.define_own_property(scope, name_chrome_desktop_capture.into(), ctor_chrome_desktop_capture.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_devtools_namespace) = tmpl_chrome_devtools_namespace.get_function(scope) {
        let name_chrome_devtools_namespace = v8::String::new(scope, "ChromeDevtoolsNamespace").unwrap();
        global.define_own_property(scope, name_chrome_devtools_namespace.into(), ctor_chrome_devtools_namespace.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_diagnostics) = tmpl_chrome_diagnostics.get_function(scope) {
        let name_chrome_diagnostics = v8::String::new(scope, "ChromeDiagnostics").unwrap();
        global.define_own_property(scope, name_chrome_diagnostics.into(), ctor_chrome_diagnostics.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_display_source) = tmpl_chrome_display_source.get_function(scope) {
        let name_chrome_display_source = v8::String::new(scope, "ChromeDisplaySource").unwrap();
        global.define_own_property(scope, name_chrome_display_source.into(), ctor_chrome_display_source.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_dns) = tmpl_chrome_dns.get_function(scope) {
        let name_chrome_dns = v8::String::new(scope, "ChromeDns").unwrap();
        global.define_own_property(scope, name_chrome_dns.into(), ctor_chrome_dns.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_document_scan) = tmpl_chrome_document_scan.get_function(scope) {
        let name_chrome_document_scan = v8::String::new(scope, "ChromeDocumentScan").unwrap();
        global.define_own_property(scope, name_chrome_document_scan.into(), ctor_chrome_document_scan.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_downloads) = tmpl_chrome_downloads.get_function(scope) {
        let name_chrome_downloads = v8::String::new(scope, "ChromeDownloads").unwrap();
        global.define_own_property(scope, name_chrome_downloads.into(), ctor_chrome_downloads.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_echo_private) = tmpl_chrome_echo_private.get_function(scope) {
        let name_chrome_echo_private = v8::String::new(scope, "ChromeEchoPrivate").unwrap();
        global.define_own_property(scope, name_chrome_echo_private.into(), ctor_chrome_echo_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_enterprise_hardware_platform) = tmpl_chrome_enterprise_hardware_platform.get_function(scope) {
        let name_chrome_enterprise_hardware_platform = v8::String::new(scope, "ChromeEnterpriseHardwarePlatform").unwrap();
        global.define_own_property(scope, name_chrome_enterprise_hardware_platform.into(), ctor_chrome_enterprise_hardware_platform.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_enterprise_ns) = tmpl_chrome_enterprise_ns.get_function(scope) {
        let name_chrome_enterprise_ns = v8::String::new(scope, "ChromeEnterpriseNS").unwrap();
        global.define_own_property(scope, name_chrome_enterprise_ns.into(), ctor_chrome_enterprise_ns.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_enterprise_platform_keys) = tmpl_chrome_enterprise_platform_keys.get_function(scope) {
        let name_chrome_enterprise_platform_keys = v8::String::new(scope, "ChromeEnterprisePlatformKeys").unwrap();
        global.define_own_property(scope, name_chrome_enterprise_platform_keys.into(), ctor_chrome_enterprise_platform_keys.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_event) = tmpl_chrome_event.get_function(scope) {
        let name_chrome_event = v8::String::new(scope, "ChromeEvent").unwrap();
        global.define_own_property(scope, name_chrome_event.into(), ctor_chrome_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_events) = tmpl_chrome_events.get_function(scope) {
        let name_chrome_events = v8::String::new(scope, "ChromeEvents").unwrap();
        global.define_own_property(scope, name_chrome_events.into(), ctor_chrome_events.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_experience_sampling_private) = tmpl_chrome_experience_sampling_private.get_function(scope) {
        let name_chrome_experience_sampling_private = v8::String::new(scope, "ChromeExperienceSamplingPrivate").unwrap();
        global.define_own_property(scope, name_chrome_experience_sampling_private.into(), ctor_chrome_experience_sampling_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_extension) = tmpl_chrome_extension.get_function(scope) {
        let name_chrome_extension = v8::String::new(scope, "ChromeExtension").unwrap();
        global.define_own_property(scope, name_chrome_extension.into(), ctor_chrome_extension.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_feedback_private) = tmpl_chrome_feedback_private.get_function(scope) {
        let name_chrome_feedback_private = v8::String::new(scope, "ChromeFeedbackPrivate").unwrap();
        global.define_own_property(scope, name_chrome_feedback_private.into(), ctor_chrome_feedback_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_file_browser_handler) = tmpl_chrome_file_browser_handler.get_function(scope) {
        let name_chrome_file_browser_handler = v8::String::new(scope, "ChromeFileBrowserHandler").unwrap();
        global.define_own_property(scope, name_chrome_file_browser_handler.into(), ctor_chrome_file_browser_handler.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_file_system_provider) = tmpl_chrome_file_system_provider.get_function(scope) {
        let name_chrome_file_system_provider = v8::String::new(scope, "ChromeFileSystemProvider").unwrap();
        global.define_own_property(scope, name_chrome_file_system_provider.into(), ctor_chrome_file_system_provider.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_first_run_private) = tmpl_chrome_first_run_private.get_function(scope) {
        let name_chrome_first_run_private = v8::String::new(scope, "ChromeFirstRunPrivate").unwrap();
        global.define_own_property(scope, name_chrome_first_run_private.into(), ctor_chrome_first_run_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_font_settings) = tmpl_chrome_font_settings.get_function(scope) {
        let name_chrome_font_settings = v8::String::new(scope, "ChromeFontSettings").unwrap();
        global.define_own_property(scope, name_chrome_font_settings.into(), ctor_chrome_font_settings.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_gcm) = tmpl_chrome_gcm.get_function(scope) {
        let name_chrome_gcm = v8::String::new(scope, "ChromeGcm").unwrap();
        global.define_own_property(scope, name_chrome_gcm.into(), ctor_chrome_gcm.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_hid) = tmpl_chrome_hid.get_function(scope) {
        let name_chrome_hid = v8::String::new(scope, "ChromeHid").unwrap();
        global.define_own_property(scope, name_chrome_hid.into(), ctor_chrome_hid.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_history) = tmpl_chrome_history.get_function(scope) {
        let name_chrome_history = v8::String::new(scope, "ChromeHistory").unwrap();
        global.define_own_property(scope, name_chrome_history.into(), ctor_chrome_history.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_hotword_private) = tmpl_chrome_hotword_private.get_function(scope) {
        let name_chrome_hotword_private = v8::String::new(scope, "ChromeHotwordPrivate").unwrap();
        global.define_own_property(scope, name_chrome_hotword_private.into(), ctor_chrome_hotword_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_i18n) = tmpl_chrome_i18n.get_function(scope) {
        let name_chrome_i18n = v8::String::new(scope, "ChromeI18n").unwrap();
        global.define_own_property(scope, name_chrome_i18n.into(), ctor_chrome_i18n.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_identity) = tmpl_chrome_identity.get_function(scope) {
        let name_chrome_identity = v8::String::new(scope, "ChromeIdentity").unwrap();
        global.define_own_property(scope, name_chrome_identity.into(), ctor_chrome_identity.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_identity_private) = tmpl_chrome_identity_private.get_function(scope) {
        let name_chrome_identity_private = v8::String::new(scope, "ChromeIdentityPrivate").unwrap();
        global.define_own_property(scope, name_chrome_identity_private.into(), ctor_chrome_identity_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_identity_private_api) = tmpl_chrome_identity_private_api.get_function(scope) {
        let name_chrome_identity_private_api = v8::String::new(scope, "ChromeIdentityPrivateAPI").unwrap();
        global.define_own_property(scope, name_chrome_identity_private_api.into(), ctor_chrome_identity_private_api.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_idle) = tmpl_chrome_idle.get_function(scope) {
        let name_chrome_idle = v8::String::new(scope, "ChromeIdle").unwrap();
        global.define_own_property(scope, name_chrome_idle.into(), ctor_chrome_idle.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_idle_private) = tmpl_chrome_idle_private.get_function(scope) {
        let name_chrome_idle_private = v8::String::new(scope, "ChromeIdlePrivate").unwrap();
        global.define_own_property(scope, name_chrome_idle_private.into(), ctor_chrome_idle_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_input_ime) = tmpl_chrome_input_ime.get_function(scope) {
        let name_chrome_input_ime = v8::String::new(scope, "ChromeInputIme").unwrap();
        global.define_own_property(scope, name_chrome_input_ime.into(), ctor_chrome_input_ime.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_input_method_private) = tmpl_chrome_input_method_private.get_function(scope) {
        let name_chrome_input_method_private = v8::String::new(scope, "ChromeInputMethodPrivate").unwrap();
        global.define_own_property(scope, name_chrome_input_method_private.into(), ctor_chrome_input_method_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_instance_id) = tmpl_chrome_instance_id.get_function(scope) {
        let name_chrome_instance_id = v8::String::new(scope, "ChromeInstanceID").unwrap();
        global.define_own_property(scope, name_chrome_instance_id.into(), ctor_chrome_instance_id.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_language_settings_private) = tmpl_chrome_language_settings_private.get_function(scope) {
        let name_chrome_language_settings_private = v8::String::new(scope, "ChromeLanguageSettingsPrivate").unwrap();
        global.define_own_property(scope, name_chrome_language_settings_private.into(), ctor_chrome_language_settings_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_load_times) = tmpl_chrome_load_times.get_function(scope) {
        let name_chrome_load_times = v8::String::new(scope, "ChromeLoadTimes").unwrap();
        global.define_own_property(scope, name_chrome_load_times.into(), ctor_chrome_load_times.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_location) = tmpl_chrome_location.get_function(scope) {
        let name_chrome_location = v8::String::new(scope, "ChromeLocation").unwrap();
        global.define_own_property(scope, name_chrome_location.into(), ctor_chrome_location.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_log_private) = tmpl_chrome_log_private.get_function(scope) {
        let name_chrome_log_private = v8::String::new(scope, "ChromeLogPrivate").unwrap();
        global.define_own_property(scope, name_chrome_log_private.into(), ctor_chrome_log_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_login_state) = tmpl_chrome_login_state.get_function(scope) {
        let name_chrome_login_state = v8::String::new(scope, "ChromeLoginState").unwrap();
        global.define_own_property(scope, name_chrome_login_state.into(), ctor_chrome_login_state.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_management) = tmpl_chrome_management.get_function(scope) {
        let name_chrome_management = v8::String::new(scope, "ChromeManagement").unwrap();
        global.define_own_property(scope, name_chrome_management.into(), ctor_chrome_management.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_mdns) = tmpl_chrome_mdns.get_function(scope) {
        let name_chrome_mdns = v8::String::new(scope, "ChromeMdns").unwrap();
        global.define_own_property(scope, name_chrome_mdns.into(), ctor_chrome_mdns.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_media_galleries) = tmpl_chrome_media_galleries.get_function(scope) {
        let name_chrome_media_galleries = v8::String::new(scope, "ChromeMediaGalleries").unwrap();
        global.define_own_property(scope, name_chrome_media_galleries.into(), ctor_chrome_media_galleries.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_media_perception_private) = tmpl_chrome_media_perception_private.get_function(scope) {
        let name_chrome_media_perception_private = v8::String::new(scope, "ChromeMediaPerceptionPrivate").unwrap();
        global.define_own_property(scope, name_chrome_media_perception_private.into(), ctor_chrome_media_perception_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_midi) = tmpl_chrome_midi.get_function(scope) {
        let name_chrome_midi = v8::String::new(scope, "ChromeMidi").unwrap();
        global.define_own_property(scope, name_chrome_midi.into(), ctor_chrome_midi.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_music_manager_private) = tmpl_chrome_music_manager_private.get_function(scope) {
        let name_chrome_music_manager_private = v8::String::new(scope, "ChromeMusicManagerPrivate").unwrap();
        global.define_own_property(scope, name_chrome_music_manager_private.into(), ctor_chrome_music_manager_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_networking_config) = tmpl_chrome_networking_config.get_function(scope) {
        let name_chrome_networking_config = v8::String::new(scope, "ChromeNetworkingConfig").unwrap();
        global.define_own_property(scope, name_chrome_networking_config.into(), ctor_chrome_networking_config.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_networking_onc) = tmpl_chrome_networking_onc.get_function(scope) {
        let name_chrome_networking_onc = v8::String::new(scope, "ChromeNetworkingOnc").unwrap();
        global.define_own_property(scope, name_chrome_networking_onc.into(), ctor_chrome_networking_onc.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_networking_private) = tmpl_chrome_networking_private.get_function(scope) {
        let name_chrome_networking_private = v8::String::new(scope, "ChromeNetworkingPrivate").unwrap();
        global.define_own_property(scope, name_chrome_networking_private.into(), ctor_chrome_networking_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_notifications) = tmpl_chrome_notifications.get_function(scope) {
        let name_chrome_notifications = v8::String::new(scope, "ChromeNotifications").unwrap();
        global.define_own_property(scope, name_chrome_notifications.into(), ctor_chrome_notifications.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_offscreen) = tmpl_chrome_offscreen.get_function(scope) {
        let name_chrome_offscreen = v8::String::new(scope, "ChromeOffscreen").unwrap();
        global.define_own_property(scope, name_chrome_offscreen.into(), ctor_chrome_offscreen.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_omnibox) = tmpl_chrome_omnibox.get_function(scope) {
        let name_chrome_omnibox = v8::String::new(scope, "ChromeOmnibox").unwrap();
        global.define_own_property(scope, name_chrome_omnibox.into(), ctor_chrome_omnibox.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_page_capture) = tmpl_chrome_page_capture.get_function(scope) {
        let name_chrome_page_capture = v8::String::new(scope, "ChromePageCapture").unwrap();
        global.define_own_property(scope, name_chrome_page_capture.into(), ctor_chrome_page_capture.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_passwords_private) = tmpl_chrome_passwords_private.get_function(scope) {
        let name_chrome_passwords_private = v8::String::new(scope, "ChromePasswordsPrivate").unwrap();
        global.define_own_property(scope, name_chrome_passwords_private.into(), ctor_chrome_passwords_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_permissions) = tmpl_chrome_permissions.get_function(scope) {
        let name_chrome_permissions = v8::String::new(scope, "ChromePermissions").unwrap();
        global.define_own_property(scope, name_chrome_permissions.into(), ctor_chrome_permissions.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_platform_keys) = tmpl_chrome_platform_keys.get_function(scope) {
        let name_chrome_platform_keys = v8::String::new(scope, "ChromePlatformKeys").unwrap();
        global.define_own_property(scope, name_chrome_platform_keys.into(), ctor_chrome_platform_keys.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_port) = tmpl_chrome_port.get_function(scope) {
        let name_chrome_port = v8::String::new(scope, "ChromePort").unwrap();
        global.define_own_property(scope, name_chrome_port.into(), ctor_chrome_port.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_power) = tmpl_chrome_power.get_function(scope) {
        let name_chrome_power = v8::String::new(scope, "ChromePower").unwrap();
        global.define_own_property(scope, name_chrome_power.into(), ctor_chrome_power.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_printer_provider) = tmpl_chrome_printer_provider.get_function(scope) {
        let name_chrome_printer_provider = v8::String::new(scope, "ChromePrinterProvider").unwrap();
        global.define_own_property(scope, name_chrome_printer_provider.into(), ctor_chrome_printer_provider.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_printing_api) = tmpl_chrome_printing_api.get_function(scope) {
        let name_chrome_printing_api = v8::String::new(scope, "ChromePrintingAPI").unwrap();
        global.define_own_property(scope, name_chrome_printing_api.into(), ctor_chrome_printing_api.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_printing_metrics) = tmpl_chrome_printing_metrics.get_function(scope) {
        let name_chrome_printing_metrics = v8::String::new(scope, "ChromePrintingMetrics").unwrap();
        global.define_own_property(scope, name_chrome_printing_metrics.into(), ctor_chrome_printing_metrics.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_privacy) = tmpl_chrome_privacy.get_function(scope) {
        let name_chrome_privacy = v8::String::new(scope, "ChromePrivacy").unwrap();
        global.define_own_property(scope, name_chrome_privacy.into(), ctor_chrome_privacy.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_processes) = tmpl_chrome_processes.get_function(scope) {
        let name_chrome_processes = v8::String::new(scope, "ChromeProcesses").unwrap();
        global.define_own_property(scope, name_chrome_processes.into(), ctor_chrome_processes.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_proxy) = tmpl_chrome_proxy.get_function(scope) {
        let name_chrome_proxy = v8::String::new(scope, "ChromeProxy").unwrap();
        global.define_own_property(scope, name_chrome_proxy.into(), ctor_chrome_proxy.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_quick_unlock_private) = tmpl_chrome_quick_unlock_private.get_function(scope) {
        let name_chrome_quick_unlock_private = v8::String::new(scope, "ChromeQuickUnlockPrivate").unwrap();
        global.define_own_property(scope, name_chrome_quick_unlock_private.into(), ctor_chrome_quick_unlock_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_reading_list) = tmpl_chrome_reading_list.get_function(scope) {
        let name_chrome_reading_list = v8::String::new(scope, "ChromeReadingList").unwrap();
        global.define_own_property(scope, name_chrome_reading_list.into(), ctor_chrome_reading_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_resource_private) = tmpl_chrome_resource_private.get_function(scope) {
        let name_chrome_resource_private = v8::String::new(scope, "ChromeResourcePrivate").unwrap();
        global.define_own_property(scope, name_chrome_resource_private.into(), ctor_chrome_resource_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_runtime) = tmpl_chrome_runtime.get_function(scope) {
        let name_chrome_runtime = v8::String::new(scope, "ChromeRuntime").unwrap();
        global.define_own_property(scope, name_chrome_runtime.into(), ctor_chrome_runtime.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_runtime_on_installed) = tmpl_chrome_runtime_on_installed.get_function(scope) {
        let name_chrome_runtime_on_installed = v8::String::new(scope, "ChromeRuntimeOnInstalled").unwrap();
        global.define_own_property(scope, name_chrome_runtime_on_installed.into(), ctor_chrome_runtime_on_installed.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_runtime_private) = tmpl_chrome_runtime_private.get_function(scope) {
        let name_chrome_runtime_private = v8::String::new(scope, "ChromeRuntimePrivate").unwrap();
        global.define_own_property(scope, name_chrome_runtime_private.into(), ctor_chrome_runtime_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_safe_browsing_private) = tmpl_chrome_safe_browsing_private.get_function(scope) {
        let name_chrome_safe_browsing_private = v8::String::new(scope, "ChromeSafeBrowsingPrivate").unwrap();
        global.define_own_property(scope, name_chrome_safe_browsing_private.into(), ctor_chrome_safe_browsing_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_scripting) = tmpl_chrome_scripting.get_function(scope) {
        let name_chrome_scripting = v8::String::new(scope, "ChromeScripting").unwrap();
        global.define_own_property(scope, name_chrome_scripting.into(), ctor_chrome_scripting.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_search) = tmpl_chrome_search.get_function(scope) {
        let name_chrome_search = v8::String::new(scope, "ChromeSearch").unwrap();
        global.define_own_property(scope, name_chrome_search.into(), ctor_chrome_search.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_serial) = tmpl_chrome_serial.get_function(scope) {
        let name_chrome_serial = v8::String::new(scope, "ChromeSerial").unwrap();
        global.define_own_property(scope, name_chrome_serial.into(), ctor_chrome_serial.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_sessions) = tmpl_chrome_sessions.get_function(scope) {
        let name_chrome_sessions = v8::String::new(scope, "ChromeSessions").unwrap();
        global.define_own_property(scope, name_chrome_sessions.into(), ctor_chrome_sessions.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_settings_private) = tmpl_chrome_settings_private.get_function(scope) {
        let name_chrome_settings_private = v8::String::new(scope, "ChromeSettingsPrivate").unwrap();
        global.define_own_property(scope, name_chrome_settings_private.into(), ctor_chrome_settings_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_side_panel) = tmpl_chrome_side_panel.get_function(scope) {
        let name_chrome_side_panel = v8::String::new(scope, "ChromeSidePanel").unwrap();
        global.define_own_property(scope, name_chrome_side_panel.into(), ctor_chrome_side_panel.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_signed_in_devices) = tmpl_chrome_signed_in_devices.get_function(scope) {
        let name_chrome_signed_in_devices = v8::String::new(scope, "ChromeSignedInDevices").unwrap();
        global.define_own_property(scope, name_chrome_signed_in_devices.into(), ctor_chrome_signed_in_devices.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_socket) = tmpl_chrome_socket.get_function(scope) {
        let name_chrome_socket = v8::String::new(scope, "ChromeSocket").unwrap();
        global.define_own_property(scope, name_chrome_socket.into(), ctor_chrome_socket.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_speech_recognition_private) = tmpl_chrome_speech_recognition_private.get_function(scope) {
        let name_chrome_speech_recognition_private = v8::String::new(scope, "ChromeSpeechRecognitionPrivate").unwrap();
        global.define_own_property(scope, name_chrome_speech_recognition_private.into(), ctor_chrome_speech_recognition_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_storage) = tmpl_chrome_storage.get_function(scope) {
        let name_chrome_storage = v8::String::new(scope, "ChromeStorage").unwrap();
        global.define_own_property(scope, name_chrome_storage.into(), ctor_chrome_storage.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_storage_area) = tmpl_chrome_storage_area.get_function(scope) {
        let name_chrome_storage_area = v8::String::new(scope, "ChromeStorageArea").unwrap();
        global.define_own_property(scope, name_chrome_storage_area.into(), ctor_chrome_storage_area.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_storage_managed) = tmpl_chrome_storage_managed.get_function(scope) {
        let name_chrome_storage_managed = v8::String::new(scope, "ChromeStorageManaged").unwrap();
        global.define_own_property(scope, name_chrome_storage_managed.into(), ctor_chrome_storage_managed.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_sync_file_system) = tmpl_chrome_sync_file_system.get_function(scope) {
        let name_chrome_sync_file_system = v8::String::new(scope, "ChromeSyncFileSystem").unwrap();
        global.define_own_property(scope, name_chrome_sync_file_system.into(), ctor_chrome_sync_file_system.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_system_cpu) = tmpl_chrome_system_cpu.get_function(scope) {
        let name_chrome_system_cpu = v8::String::new(scope, "ChromeSystemCpu").unwrap();
        global.define_own_property(scope, name_chrome_system_cpu.into(), ctor_chrome_system_cpu.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_system_display) = tmpl_chrome_system_display.get_function(scope) {
        let name_chrome_system_display = v8::String::new(scope, "ChromeSystemDisplay").unwrap();
        global.define_own_property(scope, name_chrome_system_display.into(), ctor_chrome_system_display.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_system_memory) = tmpl_chrome_system_memory.get_function(scope) {
        let name_chrome_system_memory = v8::String::new(scope, "ChromeSystemMemory").unwrap();
        global.define_own_property(scope, name_chrome_system_memory.into(), ctor_chrome_system_memory.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_system_ns) = tmpl_chrome_system_ns.get_function(scope) {
        let name_chrome_system_ns = v8::String::new(scope, "ChromeSystemNS").unwrap();
        global.define_own_property(scope, name_chrome_system_ns.into(), ctor_chrome_system_ns.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_system_network) = tmpl_chrome_system_network.get_function(scope) {
        let name_chrome_system_network = v8::String::new(scope, "ChromeSystemNetwork").unwrap();
        global.define_own_property(scope, name_chrome_system_network.into(), ctor_chrome_system_network.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_system_storage) = tmpl_chrome_system_storage.get_function(scope) {
        let name_chrome_system_storage = v8::String::new(scope, "ChromeSystemStorage").unwrap();
        global.define_own_property(scope, name_chrome_system_storage.into(), ctor_chrome_system_storage.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_tab_capture) = tmpl_chrome_tab_capture.get_function(scope) {
        let name_chrome_tab_capture = v8::String::new(scope, "ChromeTabCapture").unwrap();
        global.define_own_property(scope, name_chrome_tab_capture.into(), ctor_chrome_tab_capture.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_tab_groups) = tmpl_chrome_tab_groups.get_function(scope) {
        let name_chrome_tab_groups = v8::String::new(scope, "ChromeTabGroups").unwrap();
        global.define_own_property(scope, name_chrome_tab_groups.into(), ctor_chrome_tab_groups.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_tabs) = tmpl_chrome_tabs.get_function(scope) {
        let name_chrome_tabs = v8::String::new(scope, "ChromeTabs").unwrap();
        global.define_own_property(scope, name_chrome_tabs.into(), ctor_chrome_tabs.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_top_sites) = tmpl_chrome_top_sites.get_function(scope) {
        let name_chrome_top_sites = v8::String::new(scope, "ChromeTopSites").unwrap();
        global.define_own_property(scope, name_chrome_top_sites.into(), ctor_chrome_top_sites.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_tts) = tmpl_chrome_tts.get_function(scope) {
        let name_chrome_tts = v8::String::new(scope, "ChromeTts").unwrap();
        global.define_own_property(scope, name_chrome_tts.into(), ctor_chrome_tts.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_tts_engine) = tmpl_chrome_tts_engine.get_function(scope) {
        let name_chrome_tts_engine = v8::String::new(scope, "ChromeTtsEngine").unwrap();
        global.define_own_property(scope, name_chrome_tts_engine.into(), ctor_chrome_tts_engine.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_types) = tmpl_chrome_types.get_function(scope) {
        let name_chrome_types = v8::String::new(scope, "ChromeTypes").unwrap();
        global.define_own_property(scope, name_chrome_types.into(), ctor_chrome_types.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_usb) = tmpl_chrome_usb.get_function(scope) {
        let name_chrome_usb = v8::String::new(scope, "ChromeUsb").unwrap();
        global.define_own_property(scope, name_chrome_usb.into(), ctor_chrome_usb.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_user_scripts) = tmpl_chrome_user_scripts.get_function(scope) {
        let name_chrome_user_scripts = v8::String::new(scope, "ChromeUserScripts").unwrap();
        global.define_own_property(scope, name_chrome_user_scripts.into(), ctor_chrome_user_scripts.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_virtual_keyboard_private) = tmpl_chrome_virtual_keyboard_private.get_function(scope) {
        let name_chrome_virtual_keyboard_private = v8::String::new(scope, "ChromeVirtualKeyboardPrivate").unwrap();
        global.define_own_property(scope, name_chrome_virtual_keyboard_private.into(), ctor_chrome_virtual_keyboard_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_vpn_provider) = tmpl_chrome_vpn_provider.get_function(scope) {
        let name_chrome_vpn_provider = v8::String::new(scope, "ChromeVpnProvider").unwrap();
        global.define_own_property(scope, name_chrome_vpn_provider.into(), ctor_chrome_vpn_provider.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_wallpaper) = tmpl_chrome_wallpaper.get_function(scope) {
        let name_chrome_wallpaper = v8::String::new(scope, "ChromeWallpaper").unwrap();
        global.define_own_property(scope, name_chrome_wallpaper.into(), ctor_chrome_wallpaper.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_web_authentication_proxy) = tmpl_chrome_web_authentication_proxy.get_function(scope) {
        let name_chrome_web_authentication_proxy = v8::String::new(scope, "ChromeWebAuthenticationProxy").unwrap();
        global.define_own_property(scope, name_chrome_web_authentication_proxy.into(), ctor_chrome_web_authentication_proxy.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_web_navigation) = tmpl_chrome_web_navigation.get_function(scope) {
        let name_chrome_web_navigation = v8::String::new(scope, "ChromeWebNavigation").unwrap();
        global.define_own_property(scope, name_chrome_web_navigation.into(), ctor_chrome_web_navigation.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_web_request) = tmpl_chrome_web_request.get_function(scope) {
        let name_chrome_web_request = v8::String::new(scope, "ChromeWebRequest").unwrap();
        global.define_own_property(scope, name_chrome_web_request.into(), ctor_chrome_web_request.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_web_view) = tmpl_chrome_web_view.get_function(scope) {
        let name_chrome_web_view = v8::String::new(scope, "ChromeWebView").unwrap();
        global.define_own_property(scope, name_chrome_web_view.into(), ctor_chrome_web_view.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_webrtc_audio_private) = tmpl_chrome_webrtc_audio_private.get_function(scope) {
        let name_chrome_webrtc_audio_private = v8::String::new(scope, "ChromeWebrtcAudioPrivate").unwrap();
        global.define_own_property(scope, name_chrome_webrtc_audio_private.into(), ctor_chrome_webrtc_audio_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_webrtc_desktop_capture_private) = tmpl_chrome_webrtc_desktop_capture_private.get_function(scope) {
        let name_chrome_webrtc_desktop_capture_private = v8::String::new(scope, "ChromeWebrtcDesktopCapturePrivate").unwrap();
        global.define_own_property(scope, name_chrome_webrtc_desktop_capture_private.into(), ctor_chrome_webrtc_desktop_capture_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_webrtc_logging_private) = tmpl_chrome_webrtc_logging_private.get_function(scope) {
        let name_chrome_webrtc_logging_private = v8::String::new(scope, "ChromeWebrtcLoggingPrivate").unwrap();
        global.define_own_property(scope, name_chrome_webrtc_logging_private.into(), ctor_chrome_webrtc_logging_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_webstore) = tmpl_chrome_webstore.get_function(scope) {
        let name_chrome_webstore = v8::String::new(scope, "ChromeWebstore").unwrap();
        global.define_own_property(scope, name_chrome_webstore.into(), ctor_chrome_webstore.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_webstore_private) = tmpl_chrome_webstore_private.get_function(scope) {
        let name_chrome_webstore_private = v8::String::new(scope, "ChromeWebstorePrivate").unwrap();
        global.define_own_property(scope, name_chrome_webstore_private.into(), ctor_chrome_webstore_private.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_chrome_windows) = tmpl_chrome_windows.get_function(scope) {
        let name_chrome_windows = v8::String::new(scope, "ChromeWindows").unwrap();
        global.define_own_property(scope, name_chrome_windows.into(), ctor_chrome_windows.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_client) = tmpl_client.get_function(scope) {
        let name_client = v8::String::new(scope, "Client").unwrap();
        global.define_own_property(scope, name_client.into(), ctor_client.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_clients) = tmpl_clients.get_function(scope) {
        let name_clients = v8::String::new(scope, "Clients").unwrap();
        global.define_own_property(scope, name_clients.into(), ctor_clients.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_clipboard_item) = tmpl_clipboard_item.get_function(scope) {
        let name_clipboard_item = v8::String::new(scope, "ClipboardItem").unwrap();
        global.define_own_property(scope, name_clipboard_item.into(), ctor_clipboard_item.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_compression_stream) = tmpl_compression_stream.get_function(scope) {
        let name_compression_stream = v8::String::new(scope, "CompressionStream").unwrap();
        global.define_own_property(scope, name_compression_stream.into(), ctor_compression_stream.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_contact_address) = tmpl_contact_address.get_function(scope) {
        let name_contact_address = v8::String::new(scope, "ContactAddress").unwrap();
        global.define_own_property(scope, name_contact_address.into(), ctor_contact_address.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_contacts_manager) = tmpl_contacts_manager.get_function(scope) {
        let name_contacts_manager = v8::String::new(scope, "ContactsManager").unwrap();
        global.define_own_property(scope, name_contacts_manager.into(), ctor_contacts_manager.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_content_index) = tmpl_content_index.get_function(scope) {
        let name_content_index = v8::String::new(scope, "ContentIndex").unwrap();
        global.define_own_property(scope, name_content_index.into(), ctor_content_index.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cookie_store_manager) = tmpl_cookie_store_manager.get_function(scope) {
        let name_cookie_store_manager = v8::String::new(scope, "CookieStoreManager").unwrap();
        global.define_own_property(scope, name_cookie_store_manager.into(), ctor_cookie_store_manager.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_count_queuing_strategy) = tmpl_count_queuing_strategy.get_function(scope) {
        let name_count_queuing_strategy = v8::String::new(scope, "CountQueuingStrategy").unwrap();
        global.define_own_property(scope, name_count_queuing_strategy.into(), ctor_count_queuing_strategy.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_crash_report_context) = tmpl_crash_report_context.get_function(scope) {
        let name_crash_report_context = v8::String::new(scope, "CrashReportContext").unwrap();
        global.define_own_property(scope, name_crash_report_context.into(), ctor_crash_report_context.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_credential) = tmpl_credential.get_function(scope) {
        let name_credential = v8::String::new(scope, "Credential").unwrap();
        global.define_own_property(scope, name_credential.into(), ctor_credential.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_credentials_container) = tmpl_credentials_container.get_function(scope) {
        let name_credentials_container = v8::String::new(scope, "CredentialsContainer").unwrap();
        global.define_own_property(scope, name_credentials_container.into(), ctor_credentials_container.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_crop_target) = tmpl_crop_target.get_function(scope) {
        let name_crop_target = v8::String::new(scope, "CropTarget").unwrap();
        global.define_own_property(scope, name_crop_target.into(), ctor_crop_target.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_crypto) = tmpl_crypto.get_function(scope) {
        let name_crypto = v8::String::new(scope, "Crypto").unwrap();
        global.define_own_property(scope, name_crypto.into(), ctor_crypto.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_crypto_key) = tmpl_crypto_key.get_function(scope) {
        let name_crypto_key = v8::String::new(scope, "CryptoKey").unwrap();
        global.define_own_property(scope, name_crypto_key.into(), ctor_crypto_key.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_custom_element_registry) = tmpl_custom_element_registry.get_function(scope) {
        let name_custom_element_registry = v8::String::new(scope, "CustomElementRegistry").unwrap();
        global.define_own_property(scope, name_custom_element_registry.into(), ctor_custom_element_registry.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_custom_state_set) = tmpl_custom_state_set.get_function(scope) {
        let name_custom_state_set = v8::String::new(scope, "CustomStateSet").unwrap();
        global.define_own_property(scope, name_custom_state_set.into(), ctor_custom_state_set.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_domexception) = tmpl_domexception.get_function(scope) {
        let name_domexception = v8::String::new(scope, "DOMException").unwrap();
        global.define_own_property(scope, name_domexception.into(), ctor_domexception.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_domimplementation) = tmpl_domimplementation.get_function(scope) {
        let name_domimplementation = v8::String::new(scope, "DOMImplementation").unwrap();
        global.define_own_property(scope, name_domimplementation.into(), ctor_domimplementation.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_dommatrix_read_only) = tmpl_dommatrix_read_only.get_function(scope) {
        let name_dommatrix_read_only = v8::String::new(scope, "DOMMatrixReadOnly").unwrap();
        global.define_own_property(scope, name_dommatrix_read_only.into(), ctor_dommatrix_read_only.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_domparser) = tmpl_domparser.get_function(scope) {
        let name_domparser = v8::String::new(scope, "DOMParser").unwrap();
        global.define_own_property(scope, name_domparser.into(), ctor_domparser.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_dompoint_read_only) = tmpl_dompoint_read_only.get_function(scope) {
        let name_dompoint_read_only = v8::String::new(scope, "DOMPointReadOnly").unwrap();
        global.define_own_property(scope, name_dompoint_read_only.into(), ctor_dompoint_read_only.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_domquad) = tmpl_domquad.get_function(scope) {
        let name_domquad = v8::String::new(scope, "DOMQuad").unwrap();
        global.define_own_property(scope, name_domquad.into(), ctor_domquad.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_domrect_list) = tmpl_domrect_list.get_function(scope) {
        let name_domrect_list = v8::String::new(scope, "DOMRectList").unwrap();
        global.define_own_property(scope, name_domrect_list.into(), ctor_domrect_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_domrect_read_only) = tmpl_domrect_read_only.get_function(scope) {
        let name_domrect_read_only = v8::String::new(scope, "DOMRectReadOnly").unwrap();
        global.define_own_property(scope, name_domrect_read_only.into(), ctor_domrect_read_only.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_domstring_list) = tmpl_domstring_list.get_function(scope) {
        let name_domstring_list = v8::String::new(scope, "DOMStringList").unwrap();
        global.define_own_property(scope, name_domstring_list.into(), ctor_domstring_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_domstring_map) = tmpl_domstring_map.get_function(scope) {
        let name_domstring_map = v8::String::new(scope, "DOMStringMap").unwrap();
        global.define_own_property(scope, name_domstring_map.into(), ctor_domstring_map.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_domtoken_list) = tmpl_domtoken_list.get_function(scope) {
        let name_domtoken_list = v8::String::new(scope, "DOMTokenList").unwrap();
        global.define_own_property(scope, name_domtoken_list.into(), ctor_domtoken_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_data_transfer) = tmpl_data_transfer.get_function(scope) {
        let name_data_transfer = v8::String::new(scope, "DataTransfer").unwrap();
        global.define_own_property(scope, name_data_transfer.into(), ctor_data_transfer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_data_transfer_item) = tmpl_data_transfer_item.get_function(scope) {
        let name_data_transfer_item = v8::String::new(scope, "DataTransferItem").unwrap();
        global.define_own_property(scope, name_data_transfer_item.into(), ctor_data_transfer_item.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_data_transfer_item_list) = tmpl_data_transfer_item_list.get_function(scope) {
        let name_data_transfer_item_list = v8::String::new(scope, "DataTransferItemList").unwrap();
        global.define_own_property(scope, name_data_transfer_item_list.into(), ctor_data_transfer_item_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_decompression_stream) = tmpl_decompression_stream.get_function(scope) {
        let name_decompression_stream = v8::String::new(scope, "DecompressionStream").unwrap();
        global.define_own_property(scope, name_decompression_stream.into(), ctor_decompression_stream.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_delegated_ink_trail_presenter) = tmpl_delegated_ink_trail_presenter.get_function(scope) {
        let name_delegated_ink_trail_presenter = v8::String::new(scope, "DelegatedInkTrailPresenter").unwrap();
        global.define_own_property(scope, name_delegated_ink_trail_presenter.into(), ctor_delegated_ink_trail_presenter.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_device_motion_event_acceleration) = tmpl_device_motion_event_acceleration.get_function(scope) {
        let name_device_motion_event_acceleration = v8::String::new(scope, "DeviceMotionEventAcceleration").unwrap();
        global.define_own_property(scope, name_device_motion_event_acceleration.into(), ctor_device_motion_event_acceleration.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_device_motion_event_rotation_rate) = tmpl_device_motion_event_rotation_rate.get_function(scope) {
        let name_device_motion_event_rotation_rate = v8::String::new(scope, "DeviceMotionEventRotationRate").unwrap();
        global.define_own_property(scope, name_device_motion_event_rotation_rate.into(), ctor_device_motion_event_rotation_rate.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_digital_goods_service) = tmpl_digital_goods_service.get_function(scope) {
        let name_digital_goods_service = v8::String::new(scope, "DigitalGoodsService").unwrap();
        global.define_own_property(scope, name_digital_goods_service.into(), ctor_digital_goods_service.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    // EXT_blend_minmax: NoInterfaceObject — skip global registration
    // EXT_color_buffer_float: NoInterfaceObject — skip global registration
    // EXT_color_buffer_half_float: NoInterfaceObject — skip global registration
    // EXT_disjoint_timer_query: NoInterfaceObject — skip global registration
    // EXT_disjoint_timer_query_webgl2: NoInterfaceObject — skip global registration
    // EXT_float_blend: NoInterfaceObject — skip global registration
    // EXT_frag_depth: NoInterfaceObject — skip global registration
    // EXT_sRGB: NoInterfaceObject — skip global registration
    // EXT_shader_texture_lod: NoInterfaceObject — skip global registration
    // EXT_texture_compression_bptc: NoInterfaceObject — skip global registration
    // EXT_texture_compression_rgtc: NoInterfaceObject — skip global registration
    // EXT_texture_filter_anisotropic: NoInterfaceObject — skip global registration
    // EXT_texture_norm16: NoInterfaceObject — skip global registration
    if let Some(ctor_element_internals) = tmpl_element_internals.get_function(scope) {
        let name_element_internals = v8::String::new(scope, "ElementInternals").unwrap();
        global.define_own_property(scope, name_element_internals.into(), ctor_element_internals.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_encoded_audio_chunk) = tmpl_encoded_audio_chunk.get_function(scope) {
        let name_encoded_audio_chunk = v8::String::new(scope, "EncodedAudioChunk").unwrap();
        global.define_own_property(scope, name_encoded_audio_chunk.into(), ctor_encoded_audio_chunk.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_encoded_video_chunk) = tmpl_encoded_video_chunk.get_function(scope) {
        let name_encoded_video_chunk = v8::String::new(scope, "EncodedVideoChunk").unwrap();
        global.define_own_property(scope, name_encoded_video_chunk.into(), ctor_encoded_video_chunk.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_event) = tmpl_event.get_function(scope) {
        let name_event = v8::String::new(scope, "Event").unwrap();
        global.define_own_property(scope, name_event.into(), ctor_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_event_counts) = tmpl_event_counts.get_function(scope) {
        let name_event_counts = v8::String::new(scope, "EventCounts").unwrap();
        global.define_own_property(scope, name_event_counts.into(), ctor_event_counts.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_event_target) = tmpl_event_target.get_function(scope) {
        let name_event_target = v8::String::new(scope, "EventTarget").unwrap();
        global.define_own_property(scope, name_event_target.into(), ctor_event_target.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_exception) = tmpl_exception.get_function(scope) {
        let name_exception = v8::String::new(scope, "Exception").unwrap();
        global.define_own_property(scope, name_exception.into(), ctor_exception.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_external) = tmpl_external.get_function(scope) {
        let name_external = v8::String::new(scope, "External").unwrap();
        global.define_own_property(scope, name_external.into(), ctor_external.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_eye_dropper) = tmpl_eye_dropper.get_function(scope) {
        let name_eye_dropper = v8::String::new(scope, "EyeDropper").unwrap();
        global.define_own_property(scope, name_eye_dropper.into(), ctor_eye_dropper.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_face_detector) = tmpl_face_detector.get_function(scope) {
        let name_face_detector = v8::String::new(scope, "FaceDetector").unwrap();
        global.define_own_property(scope, name_face_detector.into(), ctor_face_detector.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_fence) = tmpl_fence.get_function(scope) {
        let name_fence = v8::String::new(scope, "Fence").unwrap();
        global.define_own_property(scope, name_fence.into(), ctor_fence.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_fenced_frame_config) = tmpl_fenced_frame_config.get_function(scope) {
        let name_fenced_frame_config = v8::String::new(scope, "FencedFrameConfig").unwrap();
        global.define_own_property(scope, name_fenced_frame_config.into(), ctor_fenced_frame_config.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_fetch_later_result) = tmpl_fetch_later_result.get_function(scope) {
        let name_fetch_later_result = v8::String::new(scope, "FetchLaterResult").unwrap();
        global.define_own_property(scope, name_fetch_later_result.into(), ctor_fetch_later_result.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_file_list) = tmpl_file_list.get_function(scope) {
        let name_file_list = v8::String::new(scope, "FileList").unwrap();
        global.define_own_property(scope, name_file_list.into(), ctor_file_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_file_reader_sync) = tmpl_file_reader_sync.get_function(scope) {
        let name_file_reader_sync = v8::String::new(scope, "FileReaderSync").unwrap();
        global.define_own_property(scope, name_file_reader_sync.into(), ctor_file_reader_sync.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_file_system) = tmpl_file_system.get_function(scope) {
        let name_file_system = v8::String::new(scope, "FileSystem").unwrap();
        global.define_own_property(scope, name_file_system.into(), ctor_file_system.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_file_system_directory_reader) = tmpl_file_system_directory_reader.get_function(scope) {
        let name_file_system_directory_reader = v8::String::new(scope, "FileSystemDirectoryReader").unwrap();
        global.define_own_property(scope, name_file_system_directory_reader.into(), ctor_file_system_directory_reader.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_file_system_entry) = tmpl_file_system_entry.get_function(scope) {
        let name_file_system_entry = v8::String::new(scope, "FileSystemEntry").unwrap();
        global.define_own_property(scope, name_file_system_entry.into(), ctor_file_system_entry.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_file_system_handle) = tmpl_file_system_handle.get_function(scope) {
        let name_file_system_handle = v8::String::new(scope, "FileSystemHandle").unwrap();
        global.define_own_property(scope, name_file_system_handle.into(), ctor_file_system_handle.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_file_system_sync_access_handle) = tmpl_file_system_sync_access_handle.get_function(scope) {
        let name_file_system_sync_access_handle = v8::String::new(scope, "FileSystemSyncAccessHandle").unwrap();
        global.define_own_property(scope, name_file_system_sync_access_handle.into(), ctor_file_system_sync_access_handle.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_font) = tmpl_font.get_function(scope) {
        let name_font = v8::String::new(scope, "Font").unwrap();
        global.define_own_property(scope, name_font.into(), ctor_font.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_font_data) = tmpl_font_data.get_function(scope) {
        let name_font_data = v8::String::new(scope, "FontData").unwrap();
        global.define_own_property(scope, name_font_data.into(), ctor_font_data.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_font_face) = tmpl_font_face.get_function(scope) {
        let name_font_face = v8::String::new(scope, "FontFace").unwrap();
        global.define_own_property(scope, name_font_face.into(), ctor_font_face.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_font_face_features) = tmpl_font_face_features.get_function(scope) {
        let name_font_face_features = v8::String::new(scope, "FontFaceFeatures").unwrap();
        global.define_own_property(scope, name_font_face_features.into(), ctor_font_face_features.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_font_face_palette) = tmpl_font_face_palette.get_function(scope) {
        let name_font_face_palette = v8::String::new(scope, "FontFacePalette").unwrap();
        global.define_own_property(scope, name_font_face_palette.into(), ctor_font_face_palette.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_font_face_palettes) = tmpl_font_face_palettes.get_function(scope) {
        let name_font_face_palettes = v8::String::new(scope, "FontFacePalettes").unwrap();
        global.define_own_property(scope, name_font_face_palettes.into(), ctor_font_face_palettes.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_font_face_variation_axis) = tmpl_font_face_variation_axis.get_function(scope) {
        let name_font_face_variation_axis = v8::String::new(scope, "FontFaceVariationAxis").unwrap();
        global.define_own_property(scope, name_font_face_variation_axis.into(), ctor_font_face_variation_axis.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_font_face_variations) = tmpl_font_face_variations.get_function(scope) {
        let name_font_face_variations = v8::String::new(scope, "FontFaceVariations").unwrap();
        global.define_own_property(scope, name_font_face_variations.into(), ctor_font_face_variations.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_font_metrics) = tmpl_font_metrics.get_function(scope) {
        let name_font_metrics = v8::String::new(scope, "FontMetrics").unwrap();
        global.define_own_property(scope, name_font_metrics.into(), ctor_font_metrics.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_form_data) = tmpl_form_data.get_function(scope) {
        let name_form_data = v8::String::new(scope, "FormData").unwrap();
        global.define_own_property(scope, name_form_data.into(), ctor_form_data.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_fragment_directive) = tmpl_fragment_directive.get_function(scope) {
        let name_fragment_directive = v8::String::new(scope, "FragmentDirective").unwrap();
        global.define_own_property(scope, name_fragment_directive.into(), ctor_fragment_directive.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_fragment_result) = tmpl_fragment_result.get_function(scope) {
        let name_fragment_result = v8::String::new(scope, "FragmentResult").unwrap();
        global.define_own_property(scope, name_fragment_result.into(), ctor_fragment_result.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu) = tmpl_gpu.get_function(scope) {
        let name_gpu = v8::String::new(scope, "GPU").unwrap();
        global.define_own_property(scope, name_gpu.into(), ctor_gpu.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpuadapter) = tmpl_gpuadapter.get_function(scope) {
        let name_gpuadapter = v8::String::new(scope, "GPUAdapter").unwrap();
        global.define_own_property(scope, name_gpuadapter.into(), ctor_gpuadapter.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpuadapter_info) = tmpl_gpuadapter_info.get_function(scope) {
        let name_gpuadapter_info = v8::String::new(scope, "GPUAdapterInfo").unwrap();
        global.define_own_property(scope, name_gpuadapter_info.into(), ctor_gpuadapter_info.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpubind_group) = tmpl_gpubind_group.get_function(scope) {
        let name_gpubind_group = v8::String::new(scope, "GPUBindGroup").unwrap();
        global.define_own_property(scope, name_gpubind_group.into(), ctor_gpubind_group.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpubind_group_layout) = tmpl_gpubind_group_layout.get_function(scope) {
        let name_gpubind_group_layout = v8::String::new(scope, "GPUBindGroupLayout").unwrap();
        global.define_own_property(scope, name_gpubind_group_layout.into(), ctor_gpubind_group_layout.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpubuffer) = tmpl_gpubuffer.get_function(scope) {
        let name_gpubuffer = v8::String::new(scope, "GPUBuffer").unwrap();
        global.define_own_property(scope, name_gpubuffer.into(), ctor_gpubuffer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpucanvas_context) = tmpl_gpucanvas_context.get_function(scope) {
        let name_gpucanvas_context = v8::String::new(scope, "GPUCanvasContext").unwrap();
        global.define_own_property(scope, name_gpucanvas_context.into(), ctor_gpucanvas_context.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpucommand_buffer) = tmpl_gpucommand_buffer.get_function(scope) {
        let name_gpucommand_buffer = v8::String::new(scope, "GPUCommandBuffer").unwrap();
        global.define_own_property(scope, name_gpucommand_buffer.into(), ctor_gpucommand_buffer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpucommand_encoder) = tmpl_gpucommand_encoder.get_function(scope) {
        let name_gpucommand_encoder = v8::String::new(scope, "GPUCommandEncoder").unwrap();
        global.define_own_property(scope, name_gpucommand_encoder.into(), ctor_gpucommand_encoder.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpucompilation_info) = tmpl_gpucompilation_info.get_function(scope) {
        let name_gpucompilation_info = v8::String::new(scope, "GPUCompilationInfo").unwrap();
        global.define_own_property(scope, name_gpucompilation_info.into(), ctor_gpucompilation_info.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpucompilation_message) = tmpl_gpucompilation_message.get_function(scope) {
        let name_gpucompilation_message = v8::String::new(scope, "GPUCompilationMessage").unwrap();
        global.define_own_property(scope, name_gpucompilation_message.into(), ctor_gpucompilation_message.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpucompute_pass_encoder) = tmpl_gpucompute_pass_encoder.get_function(scope) {
        let name_gpucompute_pass_encoder = v8::String::new(scope, "GPUComputePassEncoder").unwrap();
        global.define_own_property(scope, name_gpucompute_pass_encoder.into(), ctor_gpucompute_pass_encoder.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpucompute_pipeline) = tmpl_gpucompute_pipeline.get_function(scope) {
        let name_gpucompute_pipeline = v8::String::new(scope, "GPUComputePipeline").unwrap();
        global.define_own_property(scope, name_gpucompute_pipeline.into(), ctor_gpucompute_pipeline.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpudevice_lost_info) = tmpl_gpudevice_lost_info.get_function(scope) {
        let name_gpudevice_lost_info = v8::String::new(scope, "GPUDeviceLostInfo").unwrap();
        global.define_own_property(scope, name_gpudevice_lost_info.into(), ctor_gpudevice_lost_info.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpuerror) = tmpl_gpuerror.get_function(scope) {
        let name_gpuerror = v8::String::new(scope, "GPUError").unwrap();
        global.define_own_property(scope, name_gpuerror.into(), ctor_gpuerror.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpuexternal_texture) = tmpl_gpuexternal_texture.get_function(scope) {
        let name_gpuexternal_texture = v8::String::new(scope, "GPUExternalTexture").unwrap();
        global.define_own_property(scope, name_gpuexternal_texture.into(), ctor_gpuexternal_texture.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpupipeline_layout) = tmpl_gpupipeline_layout.get_function(scope) {
        let name_gpupipeline_layout = v8::String::new(scope, "GPUPipelineLayout").unwrap();
        global.define_own_property(scope, name_gpupipeline_layout.into(), ctor_gpupipeline_layout.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpuquery_set) = tmpl_gpuquery_set.get_function(scope) {
        let name_gpuquery_set = v8::String::new(scope, "GPUQuerySet").unwrap();
        global.define_own_property(scope, name_gpuquery_set.into(), ctor_gpuquery_set.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpuqueue) = tmpl_gpuqueue.get_function(scope) {
        let name_gpuqueue = v8::String::new(scope, "GPUQueue").unwrap();
        global.define_own_property(scope, name_gpuqueue.into(), ctor_gpuqueue.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpurender_bundle) = tmpl_gpurender_bundle.get_function(scope) {
        let name_gpurender_bundle = v8::String::new(scope, "GPURenderBundle").unwrap();
        global.define_own_property(scope, name_gpurender_bundle.into(), ctor_gpurender_bundle.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpurender_bundle_encoder) = tmpl_gpurender_bundle_encoder.get_function(scope) {
        let name_gpurender_bundle_encoder = v8::String::new(scope, "GPURenderBundleEncoder").unwrap();
        global.define_own_property(scope, name_gpurender_bundle_encoder.into(), ctor_gpurender_bundle_encoder.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpurender_pass_encoder) = tmpl_gpurender_pass_encoder.get_function(scope) {
        let name_gpurender_pass_encoder = v8::String::new(scope, "GPURenderPassEncoder").unwrap();
        global.define_own_property(scope, name_gpurender_pass_encoder.into(), ctor_gpurender_pass_encoder.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpurender_pipeline) = tmpl_gpurender_pipeline.get_function(scope) {
        let name_gpurender_pipeline = v8::String::new(scope, "GPURenderPipeline").unwrap();
        global.define_own_property(scope, name_gpurender_pipeline.into(), ctor_gpurender_pipeline.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpusampler) = tmpl_gpusampler.get_function(scope) {
        let name_gpusampler = v8::String::new(scope, "GPUSampler").unwrap();
        global.define_own_property(scope, name_gpusampler.into(), ctor_gpusampler.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpushader_module) = tmpl_gpushader_module.get_function(scope) {
        let name_gpushader_module = v8::String::new(scope, "GPUShaderModule").unwrap();
        global.define_own_property(scope, name_gpushader_module.into(), ctor_gpushader_module.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpusupported_features) = tmpl_gpusupported_features.get_function(scope) {
        let name_gpusupported_features = v8::String::new(scope, "GPUSupportedFeatures").unwrap();
        global.define_own_property(scope, name_gpusupported_features.into(), ctor_gpusupported_features.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpusupported_limits) = tmpl_gpusupported_limits.get_function(scope) {
        let name_gpusupported_limits = v8::String::new(scope, "GPUSupportedLimits").unwrap();
        global.define_own_property(scope, name_gpusupported_limits.into(), ctor_gpusupported_limits.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gputexture) = tmpl_gputexture.get_function(scope) {
        let name_gputexture = v8::String::new(scope, "GPUTexture").unwrap();
        global.define_own_property(scope, name_gputexture.into(), ctor_gputexture.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gputexture_view) = tmpl_gputexture_view.get_function(scope) {
        let name_gputexture_view = v8::String::new(scope, "GPUTextureView").unwrap();
        global.define_own_property(scope, name_gputexture_view.into(), ctor_gputexture_view.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gamepad) = tmpl_gamepad.get_function(scope) {
        let name_gamepad = v8::String::new(scope, "Gamepad").unwrap();
        global.define_own_property(scope, name_gamepad.into(), ctor_gamepad.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gamepad_button) = tmpl_gamepad_button.get_function(scope) {
        let name_gamepad_button = v8::String::new(scope, "GamepadButton").unwrap();
        global.define_own_property(scope, name_gamepad_button.into(), ctor_gamepad_button.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gamepad_haptic_actuator) = tmpl_gamepad_haptic_actuator.get_function(scope) {
        let name_gamepad_haptic_actuator = v8::String::new(scope, "GamepadHapticActuator").unwrap();
        global.define_own_property(scope, name_gamepad_haptic_actuator.into(), ctor_gamepad_haptic_actuator.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gamepad_pose) = tmpl_gamepad_pose.get_function(scope) {
        let name_gamepad_pose = v8::String::new(scope, "GamepadPose").unwrap();
        global.define_own_property(scope, name_gamepad_pose.into(), ctor_gamepad_pose.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_geolocation) = tmpl_geolocation.get_function(scope) {
        let name_geolocation = v8::String::new(scope, "Geolocation").unwrap();
        global.define_own_property(scope, name_geolocation.into(), ctor_geolocation.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_geolocation_coordinates) = tmpl_geolocation_coordinates.get_function(scope) {
        let name_geolocation_coordinates = v8::String::new(scope, "GeolocationCoordinates").unwrap();
        global.define_own_property(scope, name_geolocation_coordinates.into(), ctor_geolocation_coordinates.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_geolocation_position) = tmpl_geolocation_position.get_function(scope) {
        let name_geolocation_position = v8::String::new(scope, "GeolocationPosition").unwrap();
        global.define_own_property(scope, name_geolocation_position.into(), ctor_geolocation_position.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_geolocation_position_error) = tmpl_geolocation_position_error.get_function(scope) {
        let name_geolocation_position_error = v8::String::new(scope, "GeolocationPositionError").unwrap();
        global.define_own_property(scope, name_geolocation_position_error.into(), ctor_geolocation_position_error.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_global) = tmpl_global.get_function(scope) {
        let name_global = v8::String::new(scope, "Global").unwrap();
        global.define_own_property(scope, name_global.into(), ctor_global.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_group_effect) = tmpl_group_effect.get_function(scope) {
        let name_group_effect = v8::String::new(scope, "GroupEffect").unwrap();
        global.define_own_property(scope, name_group_effect.into(), ctor_group_effect.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlall_collection) = tmpl_htmlall_collection.get_function(scope) {
        let name_htmlall_collection = v8::String::new(scope, "HTMLAllCollection").unwrap();
        global.define_own_property(scope, name_htmlall_collection.into(), ctor_htmlall_collection.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlcollection) = tmpl_htmlcollection.get_function(scope) {
        let name_htmlcollection = v8::String::new(scope, "HTMLCollection").unwrap();
        global.define_own_property(scope, name_htmlcollection.into(), ctor_htmlcollection.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_handwriting_drawing) = tmpl_handwriting_drawing.get_function(scope) {
        let name_handwriting_drawing = v8::String::new(scope, "HandwritingDrawing").unwrap();
        global.define_own_property(scope, name_handwriting_drawing.into(), ctor_handwriting_drawing.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_handwriting_recognizer) = tmpl_handwriting_recognizer.get_function(scope) {
        let name_handwriting_recognizer = v8::String::new(scope, "HandwritingRecognizer").unwrap();
        global.define_own_property(scope, name_handwriting_recognizer.into(), ctor_handwriting_recognizer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_handwriting_stroke) = tmpl_handwriting_stroke.get_function(scope) {
        let name_handwriting_stroke = v8::String::new(scope, "HandwritingStroke").unwrap();
        global.define_own_property(scope, name_handwriting_stroke.into(), ctor_handwriting_stroke.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_headers) = tmpl_headers.get_function(scope) {
        let name_headers = v8::String::new(scope, "Headers").unwrap();
        global.define_own_property(scope, name_headers.into(), ctor_headers.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_highlight) = tmpl_highlight.get_function(scope) {
        let name_highlight = v8::String::new(scope, "Highlight").unwrap();
        global.define_own_property(scope, name_highlight.into(), ctor_highlight.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_highlight_registry) = tmpl_highlight_registry.get_function(scope) {
        let name_highlight_registry = v8::String::new(scope, "HighlightRegistry").unwrap();
        global.define_own_property(scope, name_highlight_registry.into(), ctor_highlight_registry.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_history) = tmpl_history.get_function(scope) {
        let name_history = v8::String::new(scope, "History").unwrap();
        global.define_own_property(scope, name_history.into(), ctor_history.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idbcursor) = tmpl_idbcursor.get_function(scope) {
        let name_idbcursor = v8::String::new(scope, "IDBCursor").unwrap();
        global.define_own_property(scope, name_idbcursor.into(), ctor_idbcursor.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idbfactory) = tmpl_idbfactory.get_function(scope) {
        let name_idbfactory = v8::String::new(scope, "IDBFactory").unwrap();
        global.define_own_property(scope, name_idbfactory.into(), ctor_idbfactory.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idbindex) = tmpl_idbindex.get_function(scope) {
        let name_idbindex = v8::String::new(scope, "IDBIndex").unwrap();
        global.define_own_property(scope, name_idbindex.into(), ctor_idbindex.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idbkey_range) = tmpl_idbkey_range.get_function(scope) {
        let name_idbkey_range = v8::String::new(scope, "IDBKeyRange").unwrap();
        global.define_own_property(scope, name_idbkey_range.into(), ctor_idbkey_range.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idbobject_store) = tmpl_idbobject_store.get_function(scope) {
        let name_idbobject_store = v8::String::new(scope, "IDBObjectStore").unwrap();
        global.define_own_property(scope, name_idbobject_store.into(), ctor_idbobject_store.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idbrecord) = tmpl_idbrecord.get_function(scope) {
        let name_idbrecord = v8::String::new(scope, "IDBRecord").unwrap();
        global.define_own_property(scope, name_idbrecord.into(), ctor_idbrecord.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_identity_provider) = tmpl_identity_provider.get_function(scope) {
        let name_identity_provider = v8::String::new(scope, "IdentityProvider").unwrap();
        global.define_own_property(scope, name_identity_provider.into(), ctor_identity_provider.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idle_deadline) = tmpl_idle_deadline.get_function(scope) {
        let name_idle_deadline = v8::String::new(scope, "IdleDeadline").unwrap();
        global.define_own_property(scope, name_idle_deadline.into(), ctor_idle_deadline.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_image_bitmap) = tmpl_image_bitmap.get_function(scope) {
        let name_image_bitmap = v8::String::new(scope, "ImageBitmap").unwrap();
        global.define_own_property(scope, name_image_bitmap.into(), ctor_image_bitmap.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_image_bitmap_rendering_context) = tmpl_image_bitmap_rendering_context.get_function(scope) {
        let name_image_bitmap_rendering_context = v8::String::new(scope, "ImageBitmapRenderingContext").unwrap();
        global.define_own_property(scope, name_image_bitmap_rendering_context.into(), ctor_image_bitmap_rendering_context.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_image_capture) = tmpl_image_capture.get_function(scope) {
        let name_image_capture = v8::String::new(scope, "ImageCapture").unwrap();
        global.define_own_property(scope, name_image_capture.into(), ctor_image_capture.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_image_data) = tmpl_image_data.get_function(scope) {
        let name_image_data = v8::String::new(scope, "ImageData").unwrap();
        global.define_own_property(scope, name_image_data.into(), ctor_image_data.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_image_decoder) = tmpl_image_decoder.get_function(scope) {
        let name_image_decoder = v8::String::new(scope, "ImageDecoder").unwrap();
        global.define_own_property(scope, name_image_decoder.into(), ctor_image_decoder.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_image_track) = tmpl_image_track.get_function(scope) {
        let name_image_track = v8::String::new(scope, "ImageTrack").unwrap();
        global.define_own_property(scope, name_image_track.into(), ctor_image_track.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_image_track_list) = tmpl_image_track_list.get_function(scope) {
        let name_image_track_list = v8::String::new(scope, "ImageTrackList").unwrap();
        global.define_own_property(scope, name_image_track_list.into(), ctor_image_track_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_ink) = tmpl_ink.get_function(scope) {
        let name_ink = v8::String::new(scope, "Ink").unwrap();
        global.define_own_property(scope, name_ink.into(), ctor_ink.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_input_device_capabilities) = tmpl_input_device_capabilities.get_function(scope) {
        let name_input_device_capabilities = v8::String::new(scope, "InputDeviceCapabilities").unwrap();
        global.define_own_property(scope, name_input_device_capabilities.into(), ctor_input_device_capabilities.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_instance) = tmpl_instance.get_function(scope) {
        let name_instance = v8::String::new(scope, "Instance").unwrap();
        global.define_own_property(scope, name_instance.into(), ctor_instance.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_intersection_observer) = tmpl_intersection_observer.get_function(scope) {
        let name_intersection_observer = v8::String::new(scope, "IntersectionObserver").unwrap();
        global.define_own_property(scope, name_intersection_observer.into(), ctor_intersection_observer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_intersection_observer_entry) = tmpl_intersection_observer_entry.get_function(scope) {
        let name_intersection_observer_entry = v8::String::new(scope, "IntersectionObserverEntry").unwrap();
        global.define_own_property(scope, name_intersection_observer_entry.into(), ctor_intersection_observer_entry.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_intrinsic_sizes) = tmpl_intrinsic_sizes.get_function(scope) {
        let name_intrinsic_sizes = v8::String::new(scope, "IntrinsicSizes").unwrap();
        global.define_own_property(scope, name_intrinsic_sizes.into(), ctor_intrinsic_sizes.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    // KHR_parallel_shader_compile: NoInterfaceObject — skip global registration
    if let Some(ctor_keyboard_layout_map) = tmpl_keyboard_layout_map.get_function(scope) {
        let name_keyboard_layout_map = v8::String::new(scope, "KeyboardLayoutMap").unwrap();
        global.define_own_property(scope, name_keyboard_layout_map.into(), ctor_keyboard_layout_map.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_language_detector) = tmpl_language_detector.get_function(scope) {
        let name_language_detector = v8::String::new(scope, "LanguageDetector").unwrap();
        global.define_own_property(scope, name_language_detector.into(), ctor_language_detector.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_language_model_params) = tmpl_language_model_params.get_function(scope) {
        let name_language_model_params = v8::String::new(scope, "LanguageModelParams").unwrap();
        global.define_own_property(scope, name_language_model_params.into(), ctor_language_model_params.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_launch_params) = tmpl_launch_params.get_function(scope) {
        let name_launch_params = v8::String::new(scope, "LaunchParams").unwrap();
        global.define_own_property(scope, name_launch_params.into(), ctor_launch_params.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_launch_queue) = tmpl_launch_queue.get_function(scope) {
        let name_launch_queue = v8::String::new(scope, "LaunchQueue").unwrap();
        global.define_own_property(scope, name_launch_queue.into(), ctor_launch_queue.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_layout_child) = tmpl_layout_child.get_function(scope) {
        let name_layout_child = v8::String::new(scope, "LayoutChild").unwrap();
        global.define_own_property(scope, name_layout_child.into(), ctor_layout_child.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_layout_constraints) = tmpl_layout_constraints.get_function(scope) {
        let name_layout_constraints = v8::String::new(scope, "LayoutConstraints").unwrap();
        global.define_own_property(scope, name_layout_constraints.into(), ctor_layout_constraints.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_layout_edges) = tmpl_layout_edges.get_function(scope) {
        let name_layout_edges = v8::String::new(scope, "LayoutEdges").unwrap();
        global.define_own_property(scope, name_layout_edges.into(), ctor_layout_edges.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_layout_fragment) = tmpl_layout_fragment.get_function(scope) {
        let name_layout_fragment = v8::String::new(scope, "LayoutFragment").unwrap();
        global.define_own_property(scope, name_layout_fragment.into(), ctor_layout_fragment.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_layout_shift_attribution) = tmpl_layout_shift_attribution.get_function(scope) {
        let name_layout_shift_attribution = v8::String::new(scope, "LayoutShiftAttribution").unwrap();
        global.define_own_property(scope, name_layout_shift_attribution.into(), ctor_layout_shift_attribution.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_location) = tmpl_location.get_function(scope) {
        let name_location = v8::String::new(scope, "Location").unwrap();
        global.define_own_property(scope, name_location.into(), ctor_location.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_lock) = tmpl_lock.get_function(scope) {
        let name_lock = v8::String::new(scope, "Lock").unwrap();
        global.define_own_property(scope, name_lock.into(), ctor_lock.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_lock_manager) = tmpl_lock_manager.get_function(scope) {
        let name_lock_manager = v8::String::new(scope, "LockManager").unwrap();
        global.define_own_property(scope, name_lock_manager.into(), ctor_lock_manager.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_midiinput_map) = tmpl_midiinput_map.get_function(scope) {
        let name_midiinput_map = v8::String::new(scope, "MIDIInputMap").unwrap();
        global.define_own_property(scope, name_midiinput_map.into(), ctor_midiinput_map.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_midioutput_map) = tmpl_midioutput_map.get_function(scope) {
        let name_midioutput_map = v8::String::new(scope, "MIDIOutputMap").unwrap();
        global.define_own_property(scope, name_midioutput_map.into(), ctor_midioutput_map.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_ml) = tmpl_ml.get_function(scope) {
        let name_ml = v8::String::new(scope, "ML").unwrap();
        global.define_own_property(scope, name_ml.into(), ctor_ml.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_mlcontext) = tmpl_mlcontext.get_function(scope) {
        let name_mlcontext = v8::String::new(scope, "MLContext").unwrap();
        global.define_own_property(scope, name_mlcontext.into(), ctor_mlcontext.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_mlgraph) = tmpl_mlgraph.get_function(scope) {
        let name_mlgraph = v8::String::new(scope, "MLGraph").unwrap();
        global.define_own_property(scope, name_mlgraph.into(), ctor_mlgraph.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_mlgraph_builder) = tmpl_mlgraph_builder.get_function(scope) {
        let name_mlgraph_builder = v8::String::new(scope, "MLGraphBuilder").unwrap();
        global.define_own_property(scope, name_mlgraph_builder.into(), ctor_mlgraph_builder.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_mloperand) = tmpl_mloperand.get_function(scope) {
        let name_mloperand = v8::String::new(scope, "MLOperand").unwrap();
        global.define_own_property(scope, name_mloperand.into(), ctor_mloperand.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_mltensor) = tmpl_mltensor.get_function(scope) {
        let name_mltensor = v8::String::new(scope, "MLTensor").unwrap();
        global.define_own_property(scope, name_mltensor.into(), ctor_mltensor.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_capabilities) = tmpl_media_capabilities.get_function(scope) {
        let name_media_capabilities = v8::String::new(scope, "MediaCapabilities").unwrap();
        global.define_own_property(scope, name_media_capabilities.into(), ctor_media_capabilities.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_device_info) = tmpl_media_device_info.get_function(scope) {
        let name_media_device_info = v8::String::new(scope, "MediaDeviceInfo").unwrap();
        global.define_own_property(scope, name_media_device_info.into(), ctor_media_device_info.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_error) = tmpl_media_error.get_function(scope) {
        let name_media_error = v8::String::new(scope, "MediaError").unwrap();
        global.define_own_property(scope, name_media_error.into(), ctor_media_error.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_key_status_map) = tmpl_media_key_status_map.get_function(scope) {
        let name_media_key_status_map = v8::String::new(scope, "MediaKeyStatusMap").unwrap();
        global.define_own_property(scope, name_media_key_status_map.into(), ctor_media_key_status_map.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_key_system_access) = tmpl_media_key_system_access.get_function(scope) {
        let name_media_key_system_access = v8::String::new(scope, "MediaKeySystemAccess").unwrap();
        global.define_own_property(scope, name_media_key_system_access.into(), ctor_media_key_system_access.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_keys) = tmpl_media_keys.get_function(scope) {
        let name_media_keys = v8::String::new(scope, "MediaKeys").unwrap();
        global.define_own_property(scope, name_media_keys.into(), ctor_media_keys.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_list) = tmpl_media_list.get_function(scope) {
        let name_media_list = v8::String::new(scope, "MediaList").unwrap();
        global.define_own_property(scope, name_media_list.into(), ctor_media_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_metadata) = tmpl_media_metadata.get_function(scope) {
        let name_media_metadata = v8::String::new(scope, "MediaMetadata").unwrap();
        global.define_own_property(scope, name_media_metadata.into(), ctor_media_metadata.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_session) = tmpl_media_session.get_function(scope) {
        let name_media_session = v8::String::new(scope, "MediaSession").unwrap();
        global.define_own_property(scope, name_media_session.into(), ctor_media_session.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_source_handle) = tmpl_media_source_handle.get_function(scope) {
        let name_media_source_handle = v8::String::new(scope, "MediaSourceHandle").unwrap();
        global.define_own_property(scope, name_media_source_handle.into(), ctor_media_source_handle.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_stream_track_handle) = tmpl_media_stream_track_handle.get_function(scope) {
        let name_media_stream_track_handle = v8::String::new(scope, "MediaStreamTrackHandle").unwrap();
        global.define_own_property(scope, name_media_stream_track_handle.into(), ctor_media_stream_track_handle.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_stream_track_processor) = tmpl_media_stream_track_processor.get_function(scope) {
        let name_media_stream_track_processor = v8::String::new(scope, "MediaStreamTrackProcessor").unwrap();
        global.define_own_property(scope, name_media_stream_track_processor.into(), ctor_media_stream_track_processor.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_memory) = tmpl_memory.get_function(scope) {
        let name_memory = v8::String::new(scope, "Memory").unwrap();
        global.define_own_property(scope, name_memory.into(), ctor_memory.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_message_channel) = tmpl_message_channel.get_function(scope) {
        let name_message_channel = v8::String::new(scope, "MessageChannel").unwrap();
        global.define_own_property(scope, name_message_channel.into(), ctor_message_channel.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_mime_type) = tmpl_mime_type.get_function(scope) {
        let name_mime_type = v8::String::new(scope, "MimeType").unwrap();
        global.define_own_property(scope, name_mime_type.into(), ctor_mime_type.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_mime_type_array) = tmpl_mime_type_array.get_function(scope) {
        let name_mime_type_array = v8::String::new(scope, "MimeTypeArray").unwrap();
        global.define_own_property(scope, name_mime_type_array.into(), ctor_mime_type_array.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_model_context_client) = tmpl_model_context_client.get_function(scope) {
        let name_model_context_client = v8::String::new(scope, "ModelContextClient").unwrap();
        global.define_own_property(scope, name_model_context_client.into(), ctor_model_context_client.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_module) = tmpl_module.get_function(scope) {
        let name_module = v8::String::new(scope, "Module").unwrap();
        global.define_own_property(scope, name_module.into(), ctor_module.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_mutation_observer) = tmpl_mutation_observer.get_function(scope) {
        let name_mutation_observer = v8::String::new(scope, "MutationObserver").unwrap();
        global.define_own_property(scope, name_mutation_observer.into(), ctor_mutation_observer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_mutation_record) = tmpl_mutation_record.get_function(scope) {
        let name_mutation_record = v8::String::new(scope, "MutationRecord").unwrap();
        global.define_own_property(scope, name_mutation_record.into(), ctor_mutation_record.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_ndefmessage) = tmpl_ndefmessage.get_function(scope) {
        let name_ndefmessage = v8::String::new(scope, "NDEFMessage").unwrap();
        global.define_own_property(scope, name_ndefmessage.into(), ctor_ndefmessage.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_ndefrecord) = tmpl_ndefrecord.get_function(scope) {
        let name_ndefrecord = v8::String::new(scope, "NDEFRecord").unwrap();
        global.define_own_property(scope, name_ndefrecord.into(), ctor_ndefrecord.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_named_flow_map) = tmpl_named_flow_map.get_function(scope) {
        let name_named_flow_map = v8::String::new(scope, "NamedFlowMap").unwrap();
        global.define_own_property(scope, name_named_flow_map.into(), ctor_named_flow_map.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_named_node_map) = tmpl_named_node_map.get_function(scope) {
        let name_named_node_map = v8::String::new(scope, "NamedNodeMap").unwrap();
        global.define_own_property(scope, name_named_node_map.into(), ctor_named_node_map.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_navigation_activation) = tmpl_navigation_activation.get_function(scope) {
        let name_navigation_activation = v8::String::new(scope, "NavigationActivation").unwrap();
        global.define_own_property(scope, name_navigation_activation.into(), ctor_navigation_activation.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_navigation_destination) = tmpl_navigation_destination.get_function(scope) {
        let name_navigation_destination = v8::String::new(scope, "NavigationDestination").unwrap();
        global.define_own_property(scope, name_navigation_destination.into(), ctor_navigation_destination.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_navigation_precommit_controller) = tmpl_navigation_precommit_controller.get_function(scope) {
        let name_navigation_precommit_controller = v8::String::new(scope, "NavigationPrecommitController").unwrap();
        global.define_own_property(scope, name_navigation_precommit_controller.into(), ctor_navigation_precommit_controller.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_navigation_preload_manager) = tmpl_navigation_preload_manager.get_function(scope) {
        let name_navigation_preload_manager = v8::String::new(scope, "NavigationPreloadManager").unwrap();
        global.define_own_property(scope, name_navigation_preload_manager.into(), ctor_navigation_preload_manager.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_navigation_transition) = tmpl_navigation_transition.get_function(scope) {
        let name_navigation_transition = v8::String::new(scope, "NavigationTransition").unwrap();
        global.define_own_property(scope, name_navigation_transition.into(), ctor_navigation_transition.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_navigator) = tmpl_navigator.get_function(scope) {
        let name_navigator = v8::String::new(scope, "Navigator").unwrap();
        global.define_own_property(scope, name_navigator.into(), ctor_navigator.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_navigator_login) = tmpl_navigator_login.get_function(scope) {
        let name_navigator_login = v8::String::new(scope, "NavigatorLogin").unwrap();
        global.define_own_property(scope, name_navigator_login.into(), ctor_navigator_login.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_navigator_uadata) = tmpl_navigator_uadata.get_function(scope) {
        let name_navigator_uadata = v8::String::new(scope, "NavigatorUAData").unwrap();
        global.define_own_property(scope, name_navigator_uadata.into(), ctor_navigator_uadata.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_node_iterator) = tmpl_node_iterator.get_function(scope) {
        let name_node_iterator = v8::String::new(scope, "NodeIterator").unwrap();
        global.define_own_property(scope, name_node_iterator.into(), ctor_node_iterator.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_node_list) = tmpl_node_list.get_function(scope) {
        let name_node_list = v8::String::new(scope, "NodeList").unwrap();
        global.define_own_property(scope, name_node_list.into(), ctor_node_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_not_restored_reason_details) = tmpl_not_restored_reason_details.get_function(scope) {
        let name_not_restored_reason_details = v8::String::new(scope, "NotRestoredReasonDetails").unwrap();
        global.define_own_property(scope, name_not_restored_reason_details.into(), ctor_not_restored_reason_details.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_not_restored_reasons) = tmpl_not_restored_reasons.get_function(scope) {
        let name_not_restored_reasons = v8::String::new(scope, "NotRestoredReasons").unwrap();
        global.define_own_property(scope, name_not_restored_reasons.into(), ctor_not_restored_reasons.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    // OES_draw_buffers_indexed: NoInterfaceObject — skip global registration
    // OES_element_index_uint: NoInterfaceObject — skip global registration
    // OES_fbo_render_mipmap: NoInterfaceObject — skip global registration
    // OES_standard_derivatives: NoInterfaceObject — skip global registration
    // OES_texture_float: NoInterfaceObject — skip global registration
    // OES_texture_float_linear: NoInterfaceObject — skip global registration
    // OES_texture_half_float: NoInterfaceObject — skip global registration
    // OES_texture_half_float_linear: NoInterfaceObject — skip global registration
    // OES_vertex_array_object: NoInterfaceObject — skip global registration
    // OVR_multiview2: NoInterfaceObject — skip global registration
    if let Some(ctor_observable) = tmpl_observable.get_function(scope) {
        let name_observable = v8::String::new(scope, "Observable").unwrap();
        global.define_own_property(scope, name_observable.into(), ctor_observable.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_offscreen_canvas_rendering_context2_d) = tmpl_offscreen_canvas_rendering_context2_d.get_function(scope) {
        let name_offscreen_canvas_rendering_context2_d = v8::String::new(scope, "OffscreenCanvasRenderingContext2D").unwrap();
        global.define_own_property(scope, name_offscreen_canvas_rendering_context2_d.into(), ctor_offscreen_canvas_rendering_context2_d.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_origin) = tmpl_origin.get_function(scope) {
        let name_origin = v8::String::new(scope, "Origin").unwrap();
        global.define_own_property(scope, name_origin.into(), ctor_origin.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_paint_rendering_context2_d) = tmpl_paint_rendering_context2_d.get_function(scope) {
        let name_paint_rendering_context2_d = v8::String::new(scope, "PaintRenderingContext2D").unwrap();
        global.define_own_property(scope, name_paint_rendering_context2_d.into(), ctor_paint_rendering_context2_d.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_paint_size) = tmpl_paint_size.get_function(scope) {
        let name_paint_size = v8::String::new(scope, "PaintSize").unwrap();
        global.define_own_property(scope, name_paint_size.into(), ctor_paint_size.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_path2_d) = tmpl_path2_d.get_function(scope) {
        let name_path2_d = v8::String::new(scope, "Path2D").unwrap();
        global.define_own_property(scope, name_path2_d.into(), ctor_path2_d.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_payment_manager) = tmpl_payment_manager.get_function(scope) {
        let name_payment_manager = v8::String::new(scope, "PaymentManager").unwrap();
        global.define_own_property(scope, name_payment_manager.into(), ctor_payment_manager.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_performance_entry) = tmpl_performance_entry.get_function(scope) {
        let name_performance_entry = v8::String::new(scope, "PerformanceEntry").unwrap();
        global.define_own_property(scope, name_performance_entry.into(), ctor_performance_entry.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_performance_navigation) = tmpl_performance_navigation.get_function(scope) {
        let name_performance_navigation = v8::String::new(scope, "PerformanceNavigation").unwrap();
        global.define_own_property(scope, name_performance_navigation.into(), ctor_performance_navigation.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_performance_observer) = tmpl_performance_observer.get_function(scope) {
        let name_performance_observer = v8::String::new(scope, "PerformanceObserver").unwrap();
        global.define_own_property(scope, name_performance_observer.into(), ctor_performance_observer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_performance_observer_entry_list) = tmpl_performance_observer_entry_list.get_function(scope) {
        let name_performance_observer_entry_list = v8::String::new(scope, "PerformanceObserverEntryList").unwrap();
        global.define_own_property(scope, name_performance_observer_entry_list.into(), ctor_performance_observer_entry_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_performance_server_timing) = tmpl_performance_server_timing.get_function(scope) {
        let name_performance_server_timing = v8::String::new(scope, "PerformanceServerTiming").unwrap();
        global.define_own_property(scope, name_performance_server_timing.into(), ctor_performance_server_timing.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_performance_timing) = tmpl_performance_timing.get_function(scope) {
        let name_performance_timing = v8::String::new(scope, "PerformanceTiming").unwrap();
        global.define_own_property(scope, name_performance_timing.into(), ctor_performance_timing.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_performance_timing_confidence) = tmpl_performance_timing_confidence.get_function(scope) {
        let name_performance_timing_confidence = v8::String::new(scope, "PerformanceTimingConfidence").unwrap();
        global.define_own_property(scope, name_performance_timing_confidence.into(), ctor_performance_timing_confidence.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_periodic_sync_manager) = tmpl_periodic_sync_manager.get_function(scope) {
        let name_periodic_sync_manager = v8::String::new(scope, "PeriodicSyncManager").unwrap();
        global.define_own_property(scope, name_periodic_sync_manager.into(), ctor_periodic_sync_manager.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_periodic_wave) = tmpl_periodic_wave.get_function(scope) {
        let name_periodic_wave = v8::String::new(scope, "PeriodicWave").unwrap();
        global.define_own_property(scope, name_periodic_wave.into(), ctor_periodic_wave.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_permissions) = tmpl_permissions.get_function(scope) {
        let name_permissions = v8::String::new(scope, "Permissions").unwrap();
        global.define_own_property(scope, name_permissions.into(), ctor_permissions.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_permissions_policy) = tmpl_permissions_policy.get_function(scope) {
        let name_permissions_policy = v8::String::new(scope, "PermissionsPolicy").unwrap();
        global.define_own_property(scope, name_permissions_policy.into(), ctor_permissions_policy.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_plugin) = tmpl_plugin.get_function(scope) {
        let name_plugin = v8::String::new(scope, "Plugin").unwrap();
        global.define_own_property(scope, name_plugin.into(), ctor_plugin.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_plugin_array) = tmpl_plugin_array.get_function(scope) {
        let name_plugin_array = v8::String::new(scope, "PluginArray").unwrap();
        global.define_own_property(scope, name_plugin_array.into(), ctor_plugin_array.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_preference_manager) = tmpl_preference_manager.get_function(scope) {
        let name_preference_manager = v8::String::new(scope, "PreferenceManager").unwrap();
        global.define_own_property(scope, name_preference_manager.into(), ctor_preference_manager.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_presentation) = tmpl_presentation.get_function(scope) {
        let name_presentation = v8::String::new(scope, "Presentation").unwrap();
        global.define_own_property(scope, name_presentation.into(), ctor_presentation.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_presentation_receiver) = tmpl_presentation_receiver.get_function(scope) {
        let name_presentation_receiver = v8::String::new(scope, "PresentationReceiver").unwrap();
        global.define_own_property(scope, name_presentation_receiver.into(), ctor_presentation_receiver.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_pressure_observer) = tmpl_pressure_observer.get_function(scope) {
        let name_pressure_observer = v8::String::new(scope, "PressureObserver").unwrap();
        global.define_own_property(scope, name_pressure_observer.into(), ctor_pressure_observer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_pressure_record) = tmpl_pressure_record.get_function(scope) {
        let name_pressure_record = v8::String::new(scope, "PressureRecord").unwrap();
        global.define_own_property(scope, name_pressure_record.into(), ctor_pressure_record.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_push_manager) = tmpl_push_manager.get_function(scope) {
        let name_push_manager = v8::String::new(scope, "PushManager").unwrap();
        global.define_own_property(scope, name_push_manager.into(), ctor_push_manager.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_push_message_data) = tmpl_push_message_data.get_function(scope) {
        let name_push_message_data = v8::String::new(scope, "PushMessageData").unwrap();
        global.define_own_property(scope, name_push_message_data.into(), ctor_push_message_data.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_push_subscription) = tmpl_push_subscription.get_function(scope) {
        let name_push_subscription = v8::String::new(scope, "PushSubscription").unwrap();
        global.define_own_property(scope, name_push_subscription.into(), ctor_push_subscription.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_push_subscription_options) = tmpl_push_subscription_options.get_function(scope) {
        let name_push_subscription_options = v8::String::new(scope, "PushSubscriptionOptions").unwrap();
        global.define_own_property(scope, name_push_subscription_options.into(), ctor_push_subscription_options.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtccertificate) = tmpl_rtccertificate.get_function(scope) {
        let name_rtccertificate = v8::String::new(scope, "RTCCertificate").unwrap();
        global.define_own_property(scope, name_rtccertificate.into(), ctor_rtccertificate.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcencoded_audio_frame) = tmpl_rtcencoded_audio_frame.get_function(scope) {
        let name_rtcencoded_audio_frame = v8::String::new(scope, "RTCEncodedAudioFrame").unwrap();
        global.define_own_property(scope, name_rtcencoded_audio_frame.into(), ctor_rtcencoded_audio_frame.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcencoded_video_frame) = tmpl_rtcencoded_video_frame.get_function(scope) {
        let name_rtcencoded_video_frame = v8::String::new(scope, "RTCEncodedVideoFrame").unwrap();
        global.define_own_property(scope, name_rtcencoded_video_frame.into(), ctor_rtcencoded_video_frame.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcice_candidate) = tmpl_rtcice_candidate.get_function(scope) {
        let name_rtcice_candidate = v8::String::new(scope, "RTCIceCandidate").unwrap();
        global.define_own_property(scope, name_rtcice_candidate.into(), ctor_rtcice_candidate.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcice_candidate_pair) = tmpl_rtcice_candidate_pair.get_function(scope) {
        let name_rtcice_candidate_pair = v8::String::new(scope, "RTCIceCandidatePair").unwrap();
        global.define_own_property(scope, name_rtcice_candidate_pair.into(), ctor_rtcice_candidate_pair.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcidentity_assertion) = tmpl_rtcidentity_assertion.get_function(scope) {
        let name_rtcidentity_assertion = v8::String::new(scope, "RTCIdentityAssertion").unwrap();
        global.define_own_property(scope, name_rtcidentity_assertion.into(), ctor_rtcidentity_assertion.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcidentity_provider_registrar) = tmpl_rtcidentity_provider_registrar.get_function(scope) {
        let name_rtcidentity_provider_registrar = v8::String::new(scope, "RTCIdentityProviderRegistrar").unwrap();
        global.define_own_property(scope, name_rtcidentity_provider_registrar.into(), ctor_rtcidentity_provider_registrar.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcrtp_receiver) = tmpl_rtcrtp_receiver.get_function(scope) {
        let name_rtcrtp_receiver = v8::String::new(scope, "RTCRtpReceiver").unwrap();
        global.define_own_property(scope, name_rtcrtp_receiver.into(), ctor_rtcrtp_receiver.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcrtp_sframe_encrypter) = tmpl_rtcrtp_sframe_encrypter.get_function(scope) {
        let name_rtcrtp_sframe_encrypter = v8::String::new(scope, "RTCRtpSFrameEncrypter").unwrap();
        global.define_own_property(scope, name_rtcrtp_sframe_encrypter.into(), ctor_rtcrtp_sframe_encrypter.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcrtp_script_transform) = tmpl_rtcrtp_script_transform.get_function(scope) {
        let name_rtcrtp_script_transform = v8::String::new(scope, "RTCRtpScriptTransform").unwrap();
        global.define_own_property(scope, name_rtcrtp_script_transform.into(), ctor_rtcrtp_script_transform.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcrtp_sender) = tmpl_rtcrtp_sender.get_function(scope) {
        let name_rtcrtp_sender = v8::String::new(scope, "RTCRtpSender").unwrap();
        global.define_own_property(scope, name_rtcrtp_sender.into(), ctor_rtcrtp_sender.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcrtp_transceiver) = tmpl_rtcrtp_transceiver.get_function(scope) {
        let name_rtcrtp_transceiver = v8::String::new(scope, "RTCRtpTransceiver").unwrap();
        global.define_own_property(scope, name_rtcrtp_transceiver.into(), ctor_rtcrtp_transceiver.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcsession_description) = tmpl_rtcsession_description.get_function(scope) {
        let name_rtcsession_description = v8::String::new(scope, "RTCSessionDescription").unwrap();
        global.define_own_property(scope, name_rtcsession_description.into(), ctor_rtcsession_description.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcstats_report) = tmpl_rtcstats_report.get_function(scope) {
        let name_rtcstats_report = v8::String::new(scope, "RTCStatsReport").unwrap();
        global.define_own_property(scope, name_rtcstats_report.into(), ctor_rtcstats_report.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_readable_byte_stream_controller) = tmpl_readable_byte_stream_controller.get_function(scope) {
        let name_readable_byte_stream_controller = v8::String::new(scope, "ReadableByteStreamController").unwrap();
        global.define_own_property(scope, name_readable_byte_stream_controller.into(), ctor_readable_byte_stream_controller.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_readable_stream) = tmpl_readable_stream.get_function(scope) {
        let name_readable_stream = v8::String::new(scope, "ReadableStream").unwrap();
        global.define_own_property(scope, name_readable_stream.into(), ctor_readable_stream.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_readable_stream_byobreader) = tmpl_readable_stream_byobreader.get_function(scope) {
        let name_readable_stream_byobreader = v8::String::new(scope, "ReadableStreamBYOBReader").unwrap();
        global.define_own_property(scope, name_readable_stream_byobreader.into(), ctor_readable_stream_byobreader.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_readable_stream_byobrequest) = tmpl_readable_stream_byobrequest.get_function(scope) {
        let name_readable_stream_byobrequest = v8::String::new(scope, "ReadableStreamBYOBRequest").unwrap();
        global.define_own_property(scope, name_readable_stream_byobrequest.into(), ctor_readable_stream_byobrequest.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_readable_stream_default_controller) = tmpl_readable_stream_default_controller.get_function(scope) {
        let name_readable_stream_default_controller = v8::String::new(scope, "ReadableStreamDefaultController").unwrap();
        global.define_own_property(scope, name_readable_stream_default_controller.into(), ctor_readable_stream_default_controller.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_readable_stream_default_reader) = tmpl_readable_stream_default_reader.get_function(scope) {
        let name_readable_stream_default_reader = v8::String::new(scope, "ReadableStreamDefaultReader").unwrap();
        global.define_own_property(scope, name_readable_stream_default_reader.into(), ctor_readable_stream_default_reader.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_reporting_observer) = tmpl_reporting_observer.get_function(scope) {
        let name_reporting_observer = v8::String::new(scope, "ReportingObserver").unwrap();
        global.define_own_property(scope, name_reporting_observer.into(), ctor_reporting_observer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_request) = tmpl_request.get_function(scope) {
        let name_request = v8::String::new(scope, "Request").unwrap();
        global.define_own_property(scope, name_request.into(), ctor_request.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_resize_observer) = tmpl_resize_observer.get_function(scope) {
        let name_resize_observer = v8::String::new(scope, "ResizeObserver").unwrap();
        global.define_own_property(scope, name_resize_observer.into(), ctor_resize_observer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_resize_observer_entry) = tmpl_resize_observer_entry.get_function(scope) {
        let name_resize_observer_entry = v8::String::new(scope, "ResizeObserverEntry").unwrap();
        global.define_own_property(scope, name_resize_observer_entry.into(), ctor_resize_observer_entry.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_resize_observer_size) = tmpl_resize_observer_size.get_function(scope) {
        let name_resize_observer_size = v8::String::new(scope, "ResizeObserverSize").unwrap();
        global.define_own_property(scope, name_resize_observer_size.into(), ctor_resize_observer_size.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_response) = tmpl_response.get_function(scope) {
        let name_response = v8::String::new(scope, "Response").unwrap();
        global.define_own_property(scope, name_response.into(), ctor_response.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_restriction_target) = tmpl_restriction_target.get_function(scope) {
        let name_restriction_target = v8::String::new(scope, "RestrictionTarget").unwrap();
        global.define_own_property(scope, name_restriction_target.into(), ctor_restriction_target.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rewriter) = tmpl_rewriter.get_function(scope) {
        let name_rewriter = v8::String::new(scope, "Rewriter").unwrap();
        global.define_own_property(scope, name_rewriter.into(), ctor_rewriter.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_sframe_encrypter_stream) = tmpl_sframe_encrypter_stream.get_function(scope) {
        let name_sframe_encrypter_stream = v8::String::new(scope, "SFrameEncrypterStream").unwrap();
        global.define_own_property(scope, name_sframe_encrypter_stream.into(), ctor_sframe_encrypter_stream.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgangle) = tmpl_svgangle.get_function(scope) {
        let name_svgangle = v8::String::new(scope, "SVGAngle").unwrap();
        global.define_own_property(scope, name_svgangle.into(), ctor_svgangle.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svganimated_angle) = tmpl_svganimated_angle.get_function(scope) {
        let name_svganimated_angle = v8::String::new(scope, "SVGAnimatedAngle").unwrap();
        global.define_own_property(scope, name_svganimated_angle.into(), ctor_svganimated_angle.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svganimated_boolean) = tmpl_svganimated_boolean.get_function(scope) {
        let name_svganimated_boolean = v8::String::new(scope, "SVGAnimatedBoolean").unwrap();
        global.define_own_property(scope, name_svganimated_boolean.into(), ctor_svganimated_boolean.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svganimated_enumeration) = tmpl_svganimated_enumeration.get_function(scope) {
        let name_svganimated_enumeration = v8::String::new(scope, "SVGAnimatedEnumeration").unwrap();
        global.define_own_property(scope, name_svganimated_enumeration.into(), ctor_svganimated_enumeration.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svganimated_integer) = tmpl_svganimated_integer.get_function(scope) {
        let name_svganimated_integer = v8::String::new(scope, "SVGAnimatedInteger").unwrap();
        global.define_own_property(scope, name_svganimated_integer.into(), ctor_svganimated_integer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svganimated_length) = tmpl_svganimated_length.get_function(scope) {
        let name_svganimated_length = v8::String::new(scope, "SVGAnimatedLength").unwrap();
        global.define_own_property(scope, name_svganimated_length.into(), ctor_svganimated_length.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svganimated_length_list) = tmpl_svganimated_length_list.get_function(scope) {
        let name_svganimated_length_list = v8::String::new(scope, "SVGAnimatedLengthList").unwrap();
        global.define_own_property(scope, name_svganimated_length_list.into(), ctor_svganimated_length_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svganimated_number) = tmpl_svganimated_number.get_function(scope) {
        let name_svganimated_number = v8::String::new(scope, "SVGAnimatedNumber").unwrap();
        global.define_own_property(scope, name_svganimated_number.into(), ctor_svganimated_number.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svganimated_number_list) = tmpl_svganimated_number_list.get_function(scope) {
        let name_svganimated_number_list = v8::String::new(scope, "SVGAnimatedNumberList").unwrap();
        global.define_own_property(scope, name_svganimated_number_list.into(), ctor_svganimated_number_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svganimated_preserve_aspect_ratio) = tmpl_svganimated_preserve_aspect_ratio.get_function(scope) {
        let name_svganimated_preserve_aspect_ratio = v8::String::new(scope, "SVGAnimatedPreserveAspectRatio").unwrap();
        global.define_own_property(scope, name_svganimated_preserve_aspect_ratio.into(), ctor_svganimated_preserve_aspect_ratio.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svganimated_rect) = tmpl_svganimated_rect.get_function(scope) {
        let name_svganimated_rect = v8::String::new(scope, "SVGAnimatedRect").unwrap();
        global.define_own_property(scope, name_svganimated_rect.into(), ctor_svganimated_rect.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svganimated_string) = tmpl_svganimated_string.get_function(scope) {
        let name_svganimated_string = v8::String::new(scope, "SVGAnimatedString").unwrap();
        global.define_own_property(scope, name_svganimated_string.into(), ctor_svganimated_string.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svganimated_transform_list) = tmpl_svganimated_transform_list.get_function(scope) {
        let name_svganimated_transform_list = v8::String::new(scope, "SVGAnimatedTransformList").unwrap();
        global.define_own_property(scope, name_svganimated_transform_list.into(), ctor_svganimated_transform_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svglength) = tmpl_svglength.get_function(scope) {
        let name_svglength = v8::String::new(scope, "SVGLength").unwrap();
        global.define_own_property(scope, name_svglength.into(), ctor_svglength.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svglength_list) = tmpl_svglength_list.get_function(scope) {
        let name_svglength_list = v8::String::new(scope, "SVGLengthList").unwrap();
        global.define_own_property(scope, name_svglength_list.into(), ctor_svglength_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgnumber) = tmpl_svgnumber.get_function(scope) {
        let name_svgnumber = v8::String::new(scope, "SVGNumber").unwrap();
        global.define_own_property(scope, name_svgnumber.into(), ctor_svgnumber.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgnumber_list) = tmpl_svgnumber_list.get_function(scope) {
        let name_svgnumber_list = v8::String::new(scope, "SVGNumberList").unwrap();
        global.define_own_property(scope, name_svgnumber_list.into(), ctor_svgnumber_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    // SVGPathSegment: NoInterfaceObject — skip global registration
    if let Some(ctor_svgpoint_list) = tmpl_svgpoint_list.get_function(scope) {
        let name_svgpoint_list = v8::String::new(scope, "SVGPointList").unwrap();
        global.define_own_property(scope, name_svgpoint_list.into(), ctor_svgpoint_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgpreserve_aspect_ratio) = tmpl_svgpreserve_aspect_ratio.get_function(scope) {
        let name_svgpreserve_aspect_ratio = v8::String::new(scope, "SVGPreserveAspectRatio").unwrap();
        global.define_own_property(scope, name_svgpreserve_aspect_ratio.into(), ctor_svgpreserve_aspect_ratio.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgstring_list) = tmpl_svgstring_list.get_function(scope) {
        let name_svgstring_list = v8::String::new(scope, "SVGStringList").unwrap();
        global.define_own_property(scope, name_svgstring_list.into(), ctor_svgstring_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgtransform) = tmpl_svgtransform.get_function(scope) {
        let name_svgtransform = v8::String::new(scope, "SVGTransform").unwrap();
        global.define_own_property(scope, name_svgtransform.into(), ctor_svgtransform.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgtransform_list) = tmpl_svgtransform_list.get_function(scope) {
        let name_svgtransform_list = v8::String::new(scope, "SVGTransformList").unwrap();
        global.define_own_property(scope, name_svgtransform_list.into(), ctor_svgtransform_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgunit_types) = tmpl_svgunit_types.get_function(scope) {
        let name_svgunit_types = v8::String::new(scope, "SVGUnitTypes").unwrap();
        global.define_own_property(scope, name_svgunit_types.into(), ctor_svgunit_types.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_sanitizer) = tmpl_sanitizer.get_function(scope) {
        let name_sanitizer = v8::String::new(scope, "Sanitizer").unwrap();
        global.define_own_property(scope, name_sanitizer.into(), ctor_sanitizer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_scheduler) = tmpl_scheduler.get_function(scope) {
        let name_scheduler = v8::String::new(scope, "Scheduler").unwrap();
        global.define_own_property(scope, name_scheduler.into(), ctor_scheduler.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_scheduling) = tmpl_scheduling.get_function(scope) {
        let name_scheduling = v8::String::new(scope, "Scheduling").unwrap();
        global.define_own_property(scope, name_scheduling.into(), ctor_scheduling.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_screen) = tmpl_screen.get_function(scope) {
        let name_screen = v8::String::new(scope, "Screen").unwrap();
        global.define_own_property(scope, name_screen.into(), ctor_screen.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_selection) = tmpl_selection.get_function(scope) {
        let name_selection = v8::String::new(scope, "Selection").unwrap();
        global.define_own_property(scope, name_selection.into(), ctor_selection.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_speech_grammar) = tmpl_speech_grammar.get_function(scope) {
        let name_speech_grammar = v8::String::new(scope, "SpeechGrammar").unwrap();
        global.define_own_property(scope, name_speech_grammar.into(), ctor_speech_grammar.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_speech_grammar_list) = tmpl_speech_grammar_list.get_function(scope) {
        let name_speech_grammar_list = v8::String::new(scope, "SpeechGrammarList").unwrap();
        global.define_own_property(scope, name_speech_grammar_list.into(), ctor_speech_grammar_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_speech_recognition_alternative) = tmpl_speech_recognition_alternative.get_function(scope) {
        let name_speech_recognition_alternative = v8::String::new(scope, "SpeechRecognitionAlternative").unwrap();
        global.define_own_property(scope, name_speech_recognition_alternative.into(), ctor_speech_recognition_alternative.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_speech_recognition_phrase) = tmpl_speech_recognition_phrase.get_function(scope) {
        let name_speech_recognition_phrase = v8::String::new(scope, "SpeechRecognitionPhrase").unwrap();
        global.define_own_property(scope, name_speech_recognition_phrase.into(), ctor_speech_recognition_phrase.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_speech_recognition_result) = tmpl_speech_recognition_result.get_function(scope) {
        let name_speech_recognition_result = v8::String::new(scope, "SpeechRecognitionResult").unwrap();
        global.define_own_property(scope, name_speech_recognition_result.into(), ctor_speech_recognition_result.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_speech_recognition_result_list) = tmpl_speech_recognition_result_list.get_function(scope) {
        let name_speech_recognition_result_list = v8::String::new(scope, "SpeechRecognitionResultList").unwrap();
        global.define_own_property(scope, name_speech_recognition_result_list.into(), ctor_speech_recognition_result_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_speech_synthesis_voice) = tmpl_speech_synthesis_voice.get_function(scope) {
        let name_speech_synthesis_voice = v8::String::new(scope, "SpeechSynthesisVoice").unwrap();
        global.define_own_property(scope, name_speech_synthesis_voice.into(), ctor_speech_synthesis_voice.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_storage) = tmpl_storage.get_function(scope) {
        let name_storage = v8::String::new(scope, "Storage").unwrap();
        global.define_own_property(scope, name_storage.into(), ctor_storage.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_storage_access_handle) = tmpl_storage_access_handle.get_function(scope) {
        let name_storage_access_handle = v8::String::new(scope, "StorageAccessHandle").unwrap();
        global.define_own_property(scope, name_storage_access_handle.into(), ctor_storage_access_handle.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_storage_bucket) = tmpl_storage_bucket.get_function(scope) {
        let name_storage_bucket = v8::String::new(scope, "StorageBucket").unwrap();
        global.define_own_property(scope, name_storage_bucket.into(), ctor_storage_bucket.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_storage_bucket_manager) = tmpl_storage_bucket_manager.get_function(scope) {
        let name_storage_bucket_manager = v8::String::new(scope, "StorageBucketManager").unwrap();
        global.define_own_property(scope, name_storage_bucket_manager.into(), ctor_storage_bucket_manager.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_storage_manager) = tmpl_storage_manager.get_function(scope) {
        let name_storage_manager = v8::String::new(scope, "StorageManager").unwrap();
        global.define_own_property(scope, name_storage_manager.into(), ctor_storage_manager.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_style_property_map_read_only) = tmpl_style_property_map_read_only.get_function(scope) {
        let name_style_property_map_read_only = v8::String::new(scope, "StylePropertyMapReadOnly").unwrap();
        global.define_own_property(scope, name_style_property_map_read_only.into(), ctor_style_property_map_read_only.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_style_sheet) = tmpl_style_sheet.get_function(scope) {
        let name_style_sheet = v8::String::new(scope, "StyleSheet").unwrap();
        global.define_own_property(scope, name_style_sheet.into(), ctor_style_sheet.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_style_sheet_list) = tmpl_style_sheet_list.get_function(scope) {
        let name_style_sheet_list = v8::String::new(scope, "StyleSheetList").unwrap();
        global.define_own_property(scope, name_style_sheet_list.into(), ctor_style_sheet_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_subscriber) = tmpl_subscriber.get_function(scope) {
        let name_subscriber = v8::String::new(scope, "Subscriber").unwrap();
        global.define_own_property(scope, name_subscriber.into(), ctor_subscriber.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_subtle_crypto) = tmpl_subtle_crypto.get_function(scope) {
        let name_subtle_crypto = v8::String::new(scope, "SubtleCrypto").unwrap();
        global.define_own_property(scope, name_subtle_crypto.into(), ctor_subtle_crypto.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_summarizer) = tmpl_summarizer.get_function(scope) {
        let name_summarizer = v8::String::new(scope, "Summarizer").unwrap();
        global.define_own_property(scope, name_summarizer.into(), ctor_summarizer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_sync_manager) = tmpl_sync_manager.get_function(scope) {
        let name_sync_manager = v8::String::new(scope, "SyncManager").unwrap();
        global.define_own_property(scope, name_sync_manager.into(), ctor_sync_manager.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_table) = tmpl_table.get_function(scope) {
        let name_table = v8::String::new(scope, "Table").unwrap();
        global.define_own_property(scope, name_table.into(), ctor_table.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_tag) = tmpl_tag.get_function(scope) {
        let name_tag = v8::String::new(scope, "Tag").unwrap();
        global.define_own_property(scope, name_tag.into(), ctor_tag.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_text_decoder) = tmpl_text_decoder.get_function(scope) {
        let name_text_decoder = v8::String::new(scope, "TextDecoder").unwrap();
        global.define_own_property(scope, name_text_decoder.into(), ctor_text_decoder.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_text_decoder_stream) = tmpl_text_decoder_stream.get_function(scope) {
        let name_text_decoder_stream = v8::String::new(scope, "TextDecoderStream").unwrap();
        global.define_own_property(scope, name_text_decoder_stream.into(), ctor_text_decoder_stream.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_text_detector) = tmpl_text_detector.get_function(scope) {
        let name_text_detector = v8::String::new(scope, "TextDetector").unwrap();
        global.define_own_property(scope, name_text_detector.into(), ctor_text_detector.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_text_encoder) = tmpl_text_encoder.get_function(scope) {
        let name_text_encoder = v8::String::new(scope, "TextEncoder").unwrap();
        global.define_own_property(scope, name_text_encoder.into(), ctor_text_encoder.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_text_encoder_stream) = tmpl_text_encoder_stream.get_function(scope) {
        let name_text_encoder_stream = v8::String::new(scope, "TextEncoderStream").unwrap();
        global.define_own_property(scope, name_text_encoder_stream.into(), ctor_text_encoder_stream.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_text_format) = tmpl_text_format.get_function(scope) {
        let name_text_format = v8::String::new(scope, "TextFormat").unwrap();
        global.define_own_property(scope, name_text_format.into(), ctor_text_format.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_text_metrics) = tmpl_text_metrics.get_function(scope) {
        let name_text_metrics = v8::String::new(scope, "TextMetrics").unwrap();
        global.define_own_property(scope, name_text_metrics.into(), ctor_text_metrics.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_text_track_cue_list) = tmpl_text_track_cue_list.get_function(scope) {
        let name_text_track_cue_list = v8::String::new(scope, "TextTrackCueList").unwrap();
        global.define_own_property(scope, name_text_track_cue_list.into(), ctor_text_track_cue_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_time_ranges) = tmpl_time_ranges.get_function(scope) {
        let name_time_ranges = v8::String::new(scope, "TimeRanges").unwrap();
        global.define_own_property(scope, name_time_ranges.into(), ctor_time_ranges.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_touch) = tmpl_touch.get_function(scope) {
        let name_touch = v8::String::new(scope, "Touch").unwrap();
        global.define_own_property(scope, name_touch.into(), ctor_touch.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_touch_list) = tmpl_touch_list.get_function(scope) {
        let name_touch_list = v8::String::new(scope, "TouchList").unwrap();
        global.define_own_property(scope, name_touch_list.into(), ctor_touch_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_transform_stream) = tmpl_transform_stream.get_function(scope) {
        let name_transform_stream = v8::String::new(scope, "TransformStream").unwrap();
        global.define_own_property(scope, name_transform_stream.into(), ctor_transform_stream.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_transform_stream_default_controller) = tmpl_transform_stream_default_controller.get_function(scope) {
        let name_transform_stream_default_controller = v8::String::new(scope, "TransformStreamDefaultController").unwrap();
        global.define_own_property(scope, name_transform_stream_default_controller.into(), ctor_transform_stream_default_controller.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_translator) = tmpl_translator.get_function(scope) {
        let name_translator = v8::String::new(scope, "Translator").unwrap();
        global.define_own_property(scope, name_translator.into(), ctor_translator.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_tree_walker) = tmpl_tree_walker.get_function(scope) {
        let name_tree_walker = v8::String::new(scope, "TreeWalker").unwrap();
        global.define_own_property(scope, name_tree_walker.into(), ctor_tree_walker.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_trusted_html) = tmpl_trusted_html.get_function(scope) {
        let name_trusted_html = v8::String::new(scope, "TrustedHTML").unwrap();
        global.define_own_property(scope, name_trusted_html.into(), ctor_trusted_html.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_trusted_script) = tmpl_trusted_script.get_function(scope) {
        let name_trusted_script = v8::String::new(scope, "TrustedScript").unwrap();
        global.define_own_property(scope, name_trusted_script.into(), ctor_trusted_script.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_trusted_script_url) = tmpl_trusted_script_url.get_function(scope) {
        let name_trusted_script_url = v8::String::new(scope, "TrustedScriptURL").unwrap();
        global.define_own_property(scope, name_trusted_script_url.into(), ctor_trusted_script_url.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_trusted_type_policy) = tmpl_trusted_type_policy.get_function(scope) {
        let name_trusted_type_policy = v8::String::new(scope, "TrustedTypePolicy").unwrap();
        global.define_own_property(scope, name_trusted_type_policy.into(), ctor_trusted_type_policy.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_trusted_type_policy_factory) = tmpl_trusted_type_policy_factory.get_function(scope) {
        let name_trusted_type_policy_factory = v8::String::new(scope, "TrustedTypePolicyFactory").unwrap();
        global.define_own_property(scope, name_trusted_type_policy_factory.into(), ctor_trusted_type_policy_factory.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_url) = tmpl_url.get_function(scope) {
        let name_url = v8::String::new(scope, "URL").unwrap();
        global.define_own_property(scope, name_url.into(), ctor_url.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_urlpattern) = tmpl_urlpattern.get_function(scope) {
        let name_urlpattern = v8::String::new(scope, "URLPattern").unwrap();
        global.define_own_property(scope, name_urlpattern.into(), ctor_urlpattern.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_urlsearch_params) = tmpl_urlsearch_params.get_function(scope) {
        let name_urlsearch_params = v8::String::new(scope, "URLSearchParams").unwrap();
        global.define_own_property(scope, name_urlsearch_params.into(), ctor_urlsearch_params.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usbalternate_interface) = tmpl_usbalternate_interface.get_function(scope) {
        let name_usbalternate_interface = v8::String::new(scope, "USBAlternateInterface").unwrap();
        global.define_own_property(scope, name_usbalternate_interface.into(), ctor_usbalternate_interface.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usbconfiguration) = tmpl_usbconfiguration.get_function(scope) {
        let name_usbconfiguration = v8::String::new(scope, "USBConfiguration").unwrap();
        global.define_own_property(scope, name_usbconfiguration.into(), ctor_usbconfiguration.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usbdevice) = tmpl_usbdevice.get_function(scope) {
        let name_usbdevice = v8::String::new(scope, "USBDevice").unwrap();
        global.define_own_property(scope, name_usbdevice.into(), ctor_usbdevice.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usbendpoint) = tmpl_usbendpoint.get_function(scope) {
        let name_usbendpoint = v8::String::new(scope, "USBEndpoint").unwrap();
        global.define_own_property(scope, name_usbendpoint.into(), ctor_usbendpoint.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usbin_transfer_result) = tmpl_usbin_transfer_result.get_function(scope) {
        let name_usbin_transfer_result = v8::String::new(scope, "USBInTransferResult").unwrap();
        global.define_own_property(scope, name_usbin_transfer_result.into(), ctor_usbin_transfer_result.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usbinterface) = tmpl_usbinterface.get_function(scope) {
        let name_usbinterface = v8::String::new(scope, "USBInterface").unwrap();
        global.define_own_property(scope, name_usbinterface.into(), ctor_usbinterface.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usbisochronous_in_transfer_packet) = tmpl_usbisochronous_in_transfer_packet.get_function(scope) {
        let name_usbisochronous_in_transfer_packet = v8::String::new(scope, "USBIsochronousInTransferPacket").unwrap();
        global.define_own_property(scope, name_usbisochronous_in_transfer_packet.into(), ctor_usbisochronous_in_transfer_packet.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usbisochronous_in_transfer_result) = tmpl_usbisochronous_in_transfer_result.get_function(scope) {
        let name_usbisochronous_in_transfer_result = v8::String::new(scope, "USBIsochronousInTransferResult").unwrap();
        global.define_own_property(scope, name_usbisochronous_in_transfer_result.into(), ctor_usbisochronous_in_transfer_result.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usbisochronous_out_transfer_packet) = tmpl_usbisochronous_out_transfer_packet.get_function(scope) {
        let name_usbisochronous_out_transfer_packet = v8::String::new(scope, "USBIsochronousOutTransferPacket").unwrap();
        global.define_own_property(scope, name_usbisochronous_out_transfer_packet.into(), ctor_usbisochronous_out_transfer_packet.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usbisochronous_out_transfer_result) = tmpl_usbisochronous_out_transfer_result.get_function(scope) {
        let name_usbisochronous_out_transfer_result = v8::String::new(scope, "USBIsochronousOutTransferResult").unwrap();
        global.define_own_property(scope, name_usbisochronous_out_transfer_result.into(), ctor_usbisochronous_out_transfer_result.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usbout_transfer_result) = tmpl_usbout_transfer_result.get_function(scope) {
        let name_usbout_transfer_result = v8::String::new(scope, "USBOutTransferResult").unwrap();
        global.define_own_property(scope, name_usbout_transfer_result.into(), ctor_usbout_transfer_result.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_user_activation) = tmpl_user_activation.get_function(scope) {
        let name_user_activation = v8::String::new(scope, "UserActivation").unwrap();
        global.define_own_property(scope, name_user_activation.into(), ctor_user_activation.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_vttregion) = tmpl_vttregion.get_function(scope) {
        let name_vttregion = v8::String::new(scope, "VTTRegion").unwrap();
        global.define_own_property(scope, name_vttregion.into(), ctor_vttregion.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_validity_state) = tmpl_validity_state.get_function(scope) {
        let name_validity_state = v8::String::new(scope, "ValidityState").unwrap();
        global.define_own_property(scope, name_validity_state.into(), ctor_validity_state.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_video_color_space) = tmpl_video_color_space.get_function(scope) {
        let name_video_color_space = v8::String::new(scope, "VideoColorSpace").unwrap();
        global.define_own_property(scope, name_video_color_space.into(), ctor_video_color_space.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_video_frame) = tmpl_video_frame.get_function(scope) {
        let name_video_frame = v8::String::new(scope, "VideoFrame").unwrap();
        global.define_own_property(scope, name_video_frame.into(), ctor_video_frame.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_video_playback_quality) = tmpl_video_playback_quality.get_function(scope) {
        let name_video_playback_quality = v8::String::new(scope, "VideoPlaybackQuality").unwrap();
        global.define_own_property(scope, name_video_playback_quality.into(), ctor_video_playback_quality.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_video_track) = tmpl_video_track.get_function(scope) {
        let name_video_track = v8::String::new(scope, "VideoTrack").unwrap();
        global.define_own_property(scope, name_video_track.into(), ctor_video_track.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_video_track_generator) = tmpl_video_track_generator.get_function(scope) {
        let name_video_track_generator = v8::String::new(scope, "VideoTrackGenerator").unwrap();
        global.define_own_property(scope, name_video_track_generator.into(), ctor_video_track_generator.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_view_transition) = tmpl_view_transition.get_function(scope) {
        let name_view_transition = v8::String::new(scope, "ViewTransition").unwrap();
        global.define_own_property(scope, name_view_transition.into(), ctor_view_transition.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_view_transition_type_set) = tmpl_view_transition_type_set.get_function(scope) {
        let name_view_transition_type_set = v8::String::new(scope, "ViewTransitionTypeSet").unwrap();
        global.define_own_property(scope, name_view_transition_type_set.into(), ctor_view_transition_type_set.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_viewport) = tmpl_viewport.get_function(scope) {
        let name_viewport = v8::String::new(scope, "Viewport").unwrap();
        global.define_own_property(scope, name_viewport.into(), ctor_viewport.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    // WEBGL_blend_equation_advanced_coherent: NoInterfaceObject — skip global registration
    // WEBGL_clip_cull_distance: NoInterfaceObject — skip global registration
    // WEBGL_color_buffer_float: NoInterfaceObject — skip global registration
    // WEBGL_compressed_texture_astc: NoInterfaceObject — skip global registration
    // WEBGL_compressed_texture_etc: NoInterfaceObject — skip global registration
    // WEBGL_compressed_texture_etc1: NoInterfaceObject — skip global registration
    // WEBGL_compressed_texture_pvrtc: NoInterfaceObject — skip global registration
    // WEBGL_compressed_texture_s3tc: NoInterfaceObject — skip global registration
    // WEBGL_compressed_texture_s3tc_srgb: NoInterfaceObject — skip global registration
    // WEBGL_debug_renderer_info: NoInterfaceObject — skip global registration
    // WEBGL_debug_shaders: NoInterfaceObject — skip global registration
    // WEBGL_depth_texture: NoInterfaceObject — skip global registration
    // WEBGL_draw_buffers: NoInterfaceObject — skip global registration
    // WEBGL_draw_instanced_base_vertex_base_instance: NoInterfaceObject — skip global registration
    // WEBGL_lose_context: NoInterfaceObject — skip global registration
    // WEBGL_multi_draw: NoInterfaceObject — skip global registration
    // WEBGL_multi_draw_instanced_base_vertex_base_instance: NoInterfaceObject — skip global registration
    // WEBGL_provoking_vertex: NoInterfaceObject — skip global registration
    if let Some(ctor_wgsllanguage_features) = tmpl_wgsllanguage_features.get_function(scope) {
        let name_wgsllanguage_features = v8::String::new(scope, "WGSLLanguageFeatures").unwrap();
        global.define_own_property(scope, name_wgsllanguage_features.into(), ctor_wgsllanguage_features.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_wake_lock) = tmpl_wake_lock.get_function(scope) {
        let name_wake_lock = v8::String::new(scope, "WakeLock").unwrap();
        global.define_own_property(scope, name_wake_lock.into(), ctor_wake_lock.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_gl2_rendering_context) = tmpl_web_gl2_rendering_context.get_function(scope) {
        let name_web_gl2_rendering_context = v8::String::new(scope, "WebGL2RenderingContext").unwrap();
        global.define_own_property(scope, name_web_gl2_rendering_context.into(), ctor_web_gl2_rendering_context.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_glactive_info) = tmpl_web_glactive_info.get_function(scope) {
        let name_web_glactive_info = v8::String::new(scope, "WebGLActiveInfo").unwrap();
        global.define_own_property(scope, name_web_glactive_info.into(), ctor_web_glactive_info.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_globject) = tmpl_web_globject.get_function(scope) {
        let name_web_globject = v8::String::new(scope, "WebGLObject").unwrap();
        global.define_own_property(scope, name_web_globject.into(), ctor_web_globject.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_glrendering_context) = tmpl_web_glrendering_context.get_function(scope) {
        let name_web_glrendering_context = v8::String::new(scope, "WebGLRenderingContext").unwrap();
        global.define_own_property(scope, name_web_glrendering_context.into(), ctor_web_glrendering_context.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_glshader_precision_format) = tmpl_web_glshader_precision_format.get_function(scope) {
        let name_web_glshader_precision_format = v8::String::new(scope, "WebGLShaderPrecisionFormat").unwrap();
        global.define_own_property(scope, name_web_glshader_precision_format.into(), ctor_web_glshader_precision_format.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_gluniform_location) = tmpl_web_gluniform_location.get_function(scope) {
        let name_web_gluniform_location = v8::String::new(scope, "WebGLUniformLocation").unwrap();
        global.define_own_property(scope, name_web_gluniform_location.into(), ctor_web_gluniform_location.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_transport) = tmpl_web_transport.get_function(scope) {
        let name_web_transport = v8::String::new(scope, "WebTransport").unwrap();
        global.define_own_property(scope, name_web_transport.into(), ctor_web_transport.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_transport_bidirectional_stream) = tmpl_web_transport_bidirectional_stream.get_function(scope) {
        let name_web_transport_bidirectional_stream = v8::String::new(scope, "WebTransportBidirectionalStream").unwrap();
        global.define_own_property(scope, name_web_transport_bidirectional_stream.into(), ctor_web_transport_bidirectional_stream.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_transport_datagram_duplex_stream) = tmpl_web_transport_datagram_duplex_stream.get_function(scope) {
        let name_web_transport_datagram_duplex_stream = v8::String::new(scope, "WebTransportDatagramDuplexStream").unwrap();
        global.define_own_property(scope, name_web_transport_datagram_duplex_stream.into(), ctor_web_transport_datagram_duplex_stream.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_transport_send_group) = tmpl_web_transport_send_group.get_function(scope) {
        let name_web_transport_send_group = v8::String::new(scope, "WebTransportSendGroup").unwrap();
        global.define_own_property(scope, name_web_transport_send_group.into(), ctor_web_transport_send_group.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_worker_location) = tmpl_worker_location.get_function(scope) {
        let name_worker_location = v8::String::new(scope, "WorkerLocation").unwrap();
        global.define_own_property(scope, name_worker_location.into(), ctor_worker_location.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_worker_navigator) = tmpl_worker_navigator.get_function(scope) {
        let name_worker_navigator = v8::String::new(scope, "WorkerNavigator").unwrap();
        global.define_own_property(scope, name_worker_navigator.into(), ctor_worker_navigator.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_worklet) = tmpl_worklet.get_function(scope) {
        let name_worklet = v8::String::new(scope, "Worklet").unwrap();
        global.define_own_property(scope, name_worklet.into(), ctor_worklet.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_worklet_animation_effect) = tmpl_worklet_animation_effect.get_function(scope) {
        let name_worklet_animation_effect = v8::String::new(scope, "WorkletAnimationEffect").unwrap();
        global.define_own_property(scope, name_worklet_animation_effect.into(), ctor_worklet_animation_effect.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_worklet_global_scope) = tmpl_worklet_global_scope.get_function(scope) {
        let name_worklet_global_scope = v8::String::new(scope, "WorkletGlobalScope").unwrap();
        global.define_own_property(scope, name_worklet_global_scope.into(), ctor_worklet_global_scope.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_worklet_group_effect) = tmpl_worklet_group_effect.get_function(scope) {
        let name_worklet_group_effect = v8::String::new(scope, "WorkletGroupEffect").unwrap();
        global.define_own_property(scope, name_worklet_group_effect.into(), ctor_worklet_group_effect.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_writable_stream) = tmpl_writable_stream.get_function(scope) {
        let name_writable_stream = v8::String::new(scope, "WritableStream").unwrap();
        global.define_own_property(scope, name_writable_stream.into(), ctor_writable_stream.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_writable_stream_default_controller) = tmpl_writable_stream_default_controller.get_function(scope) {
        let name_writable_stream_default_controller = v8::String::new(scope, "WritableStreamDefaultController").unwrap();
        global.define_own_property(scope, name_writable_stream_default_controller.into(), ctor_writable_stream_default_controller.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_writable_stream_default_writer) = tmpl_writable_stream_default_writer.get_function(scope) {
        let name_writable_stream_default_writer = v8::String::new(scope, "WritableStreamDefaultWriter").unwrap();
        global.define_own_property(scope, name_writable_stream_default_writer.into(), ctor_writable_stream_default_writer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_writer) = tmpl_writer.get_function(scope) {
        let name_writer = v8::String::new(scope, "Writer").unwrap();
        global.define_own_property(scope, name_writer.into(), ctor_writer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xmlserializer) = tmpl_xmlserializer.get_function(scope) {
        let name_xmlserializer = v8::String::new(scope, "XMLSerializer").unwrap();
        global.define_own_property(scope, name_xmlserializer.into(), ctor_xmlserializer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xpath_evaluator) = tmpl_xpath_evaluator.get_function(scope) {
        let name_xpath_evaluator = v8::String::new(scope, "XPathEvaluator").unwrap();
        global.define_own_property(scope, name_xpath_evaluator.into(), ctor_xpath_evaluator.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xpath_expression) = tmpl_xpath_expression.get_function(scope) {
        let name_xpath_expression = v8::String::new(scope, "XPathExpression").unwrap();
        global.define_own_property(scope, name_xpath_expression.into(), ctor_xpath_expression.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xpath_result) = tmpl_xpath_result.get_function(scope) {
        let name_xpath_result = v8::String::new(scope, "XPathResult").unwrap();
        global.define_own_property(scope, name_xpath_result.into(), ctor_xpath_result.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xranchor) = tmpl_xranchor.get_function(scope) {
        let name_xranchor = v8::String::new(scope, "XRAnchor").unwrap();
        global.define_own_property(scope, name_xranchor.into(), ctor_xranchor.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xranchor_set) = tmpl_xranchor_set.get_function(scope) {
        let name_xranchor_set = v8::String::new(scope, "XRAnchorSet").unwrap();
        global.define_own_property(scope, name_xranchor_set.into(), ctor_xranchor_set.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrbody) = tmpl_xrbody.get_function(scope) {
        let name_xrbody = v8::String::new(scope, "XRBody").unwrap();
        global.define_own_property(scope, name_xrbody.into(), ctor_xrbody.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrcamera) = tmpl_xrcamera.get_function(scope) {
        let name_xrcamera = v8::String::new(scope, "XRCamera").unwrap();
        global.define_own_property(scope, name_xrcamera.into(), ctor_xrcamera.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrdepth_information) = tmpl_xrdepth_information.get_function(scope) {
        let name_xrdepth_information = v8::String::new(scope, "XRDepthInformation").unwrap();
        global.define_own_property(scope, name_xrdepth_information.into(), ctor_xrdepth_information.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrframe) = tmpl_xrframe.get_function(scope) {
        let name_xrframe = v8::String::new(scope, "XRFrame").unwrap();
        global.define_own_property(scope, name_xrframe.into(), ctor_xrframe.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrhand) = tmpl_xrhand.get_function(scope) {
        let name_xrhand = v8::String::new(scope, "XRHand").unwrap();
        global.define_own_property(scope, name_xrhand.into(), ctor_xrhand.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrhit_test_result) = tmpl_xrhit_test_result.get_function(scope) {
        let name_xrhit_test_result = v8::String::new(scope, "XRHitTestResult").unwrap();
        global.define_own_property(scope, name_xrhit_test_result.into(), ctor_xrhit_test_result.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrhit_test_source) = tmpl_xrhit_test_source.get_function(scope) {
        let name_xrhit_test_source = v8::String::new(scope, "XRHitTestSource").unwrap();
        global.define_own_property(scope, name_xrhit_test_source.into(), ctor_xrhit_test_source.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrinput_source) = tmpl_xrinput_source.get_function(scope) {
        let name_xrinput_source = v8::String::new(scope, "XRInputSource").unwrap();
        global.define_own_property(scope, name_xrinput_source.into(), ctor_xrinput_source.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrinput_source_array) = tmpl_xrinput_source_array.get_function(scope) {
        let name_xrinput_source_array = v8::String::new(scope, "XRInputSourceArray").unwrap();
        global.define_own_property(scope, name_xrinput_source_array.into(), ctor_xrinput_source_array.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrlight_estimate) = tmpl_xrlight_estimate.get_function(scope) {
        let name_xrlight_estimate = v8::String::new(scope, "XRLightEstimate").unwrap();
        global.define_own_property(scope, name_xrlight_estimate.into(), ctor_xrlight_estimate.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrmedia_binding) = tmpl_xrmedia_binding.get_function(scope) {
        let name_xrmedia_binding = v8::String::new(scope, "XRMediaBinding").unwrap();
        global.define_own_property(scope, name_xrmedia_binding.into(), ctor_xrmedia_binding.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrmesh) = tmpl_xrmesh.get_function(scope) {
        let name_xrmesh = v8::String::new(scope, "XRMesh").unwrap();
        global.define_own_property(scope, name_xrmesh.into(), ctor_xrmesh.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrmesh_set) = tmpl_xrmesh_set.get_function(scope) {
        let name_xrmesh_set = v8::String::new(scope, "XRMeshSet").unwrap();
        global.define_own_property(scope, name_xrmesh_set.into(), ctor_xrmesh_set.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrplane) = tmpl_xrplane.get_function(scope) {
        let name_xrplane = v8::String::new(scope, "XRPlane").unwrap();
        global.define_own_property(scope, name_xrplane.into(), ctor_xrplane.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrplane_set) = tmpl_xrplane_set.get_function(scope) {
        let name_xrplane_set = v8::String::new(scope, "XRPlaneSet").unwrap();
        global.define_own_property(scope, name_xrplane_set.into(), ctor_xrplane_set.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrpose) = tmpl_xrpose.get_function(scope) {
        let name_xrpose = v8::String::new(scope, "XRPose").unwrap();
        global.define_own_property(scope, name_xrpose.into(), ctor_xrpose.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrray) = tmpl_xrray.get_function(scope) {
        let name_xrray = v8::String::new(scope, "XRRay").unwrap();
        global.define_own_property(scope, name_xrray.into(), ctor_xrray.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrrender_state) = tmpl_xrrender_state.get_function(scope) {
        let name_xrrender_state = v8::String::new(scope, "XRRenderState").unwrap();
        global.define_own_property(scope, name_xrrender_state.into(), ctor_xrrender_state.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrrigid_transform) = tmpl_xrrigid_transform.get_function(scope) {
        let name_xrrigid_transform = v8::String::new(scope, "XRRigidTransform").unwrap();
        global.define_own_property(scope, name_xrrigid_transform.into(), ctor_xrrigid_transform.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrsub_image) = tmpl_xrsub_image.get_function(scope) {
        let name_xrsub_image = v8::String::new(scope, "XRSubImage").unwrap();
        global.define_own_property(scope, name_xrsub_image.into(), ctor_xrsub_image.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrtransient_input_hit_test_result) = tmpl_xrtransient_input_hit_test_result.get_function(scope) {
        let name_xrtransient_input_hit_test_result = v8::String::new(scope, "XRTransientInputHitTestResult").unwrap();
        global.define_own_property(scope, name_xrtransient_input_hit_test_result.into(), ctor_xrtransient_input_hit_test_result.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrtransient_input_hit_test_source) = tmpl_xrtransient_input_hit_test_source.get_function(scope) {
        let name_xrtransient_input_hit_test_source = v8::String::new(scope, "XRTransientInputHitTestSource").unwrap();
        global.define_own_property(scope, name_xrtransient_input_hit_test_source.into(), ctor_xrtransient_input_hit_test_source.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrview) = tmpl_xrview.get_function(scope) {
        let name_xrview = v8::String::new(scope, "XRView").unwrap();
        global.define_own_property(scope, name_xrview.into(), ctor_xrview.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrviewport) = tmpl_xrviewport.get_function(scope) {
        let name_xrviewport = v8::String::new(scope, "XRViewport").unwrap();
        global.define_own_property(scope, name_xrviewport.into(), ctor_xrviewport.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrweb_glbinding) = tmpl_xrweb_glbinding.get_function(scope) {
        let name_xrweb_glbinding = v8::String::new(scope, "XRWebGLBinding").unwrap();
        global.define_own_property(scope, name_xrweb_glbinding.into(), ctor_xrweb_glbinding.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xsltprocessor) = tmpl_xsltprocessor.get_function(scope) {
        let name_xsltprocessor = v8::String::new(scope, "XSLTProcessor").unwrap();
        global.define_own_property(scope, name_xsltprocessor.into(), ctor_xsltprocessor.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_webkit_cancel_animation_frame) = tmpl_webkit_cancel_animation_frame.get_function(scope) {
        let name_webkit_cancel_animation_frame = v8::String::new(scope, "webkitCancelAnimationFrame").unwrap();
        global.define_own_property(scope, name_webkit_cancel_animation_frame.into(), ctor_webkit_cancel_animation_frame.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_webkit_convert_point_from_node_to_page) = tmpl_webkit_convert_point_from_node_to_page.get_function(scope) {
        let name_webkit_convert_point_from_node_to_page = v8::String::new(scope, "webkitConvertPointFromNodeToPage").unwrap();
        global.define_own_property(scope, name_webkit_convert_point_from_node_to_page.into(), ctor_webkit_convert_point_from_node_to_page.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_webkit_convert_point_from_page_to_node) = tmpl_webkit_convert_point_from_page_to_node.get_function(scope) {
        let name_webkit_convert_point_from_page_to_node = v8::String::new(scope, "webkitConvertPointFromPageToNode").unwrap();
        global.define_own_property(scope, name_webkit_convert_point_from_page_to_node.into(), ctor_webkit_convert_point_from_page_to_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_webkit_get_gamepads) = tmpl_webkit_get_gamepads.get_function(scope) {
        let name_webkit_get_gamepads = v8::String::new(scope, "webkitGetGamepads").unwrap();
        global.define_own_property(scope, name_webkit_get_gamepads.into(), ctor_webkit_get_gamepads.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_webkit_get_user_media) = tmpl_webkit_get_user_media.get_function(scope) {
        let name_webkit_get_user_media = v8::String::new(scope, "webkitGetUserMedia").unwrap();
        global.define_own_property(scope, name_webkit_get_user_media.into(), ctor_webkit_get_user_media.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_webkit_idbkey_range) = tmpl_webkit_idbkey_range.get_function(scope) {
        let name_webkit_idbkey_range = v8::String::new(scope, "webkitIDBKeyRange").unwrap();
        global.define_own_property(scope, name_webkit_idbkey_range.into(), ctor_webkit_idbkey_range.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_webkit_indexed_db) = tmpl_webkit_indexed_db.get_function(scope) {
        let name_webkit_indexed_db = v8::String::new(scope, "webkitIndexedDB").unwrap();
        global.define_own_property(scope, name_webkit_indexed_db.into(), ctor_webkit_indexed_db.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_webkit_match_media) = tmpl_webkit_match_media.get_function(scope) {
        let name_webkit_match_media = v8::String::new(scope, "webkitMatchMedia").unwrap();
        global.define_own_property(scope, name_webkit_match_media.into(), ctor_webkit_match_media.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_webkit_media_stream) = tmpl_webkit_media_stream.get_function(scope) {
        let name_webkit_media_stream = v8::String::new(scope, "webkitMediaStream").unwrap();
        global.define_own_property(scope, name_webkit_media_stream.into(), ctor_webkit_media_stream.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_webkit_notifications) = tmpl_webkit_notifications.get_function(scope) {
        let name_webkit_notifications = v8::String::new(scope, "webkitNotifications").unwrap();
        global.define_own_property(scope, name_webkit_notifications.into(), ctor_webkit_notifications.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_webkit_performance) = tmpl_webkit_performance.get_function(scope) {
        let name_webkit_performance = v8::String::new(scope, "webkitPerformance").unwrap();
        global.define_own_property(scope, name_webkit_performance.into(), ctor_webkit_performance.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_webkit_rtcpeer_connection) = tmpl_webkit_rtcpeer_connection.get_function(scope) {
        let name_webkit_rtcpeer_connection = v8::String::new(scope, "webkitRTCPeerConnection").unwrap();
        global.define_own_property(scope, name_webkit_rtcpeer_connection.into(), ctor_webkit_rtcpeer_connection.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_webkit_request_animation_frame) = tmpl_webkit_request_animation_frame.get_function(scope) {
        let name_webkit_request_animation_frame = v8::String::new(scope, "webkitRequestAnimationFrame").unwrap();
        global.define_own_property(scope, name_webkit_request_animation_frame.into(), ctor_webkit_request_animation_frame.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_webkit_request_file_system) = tmpl_webkit_request_file_system.get_function(scope) {
        let name_webkit_request_file_system = v8::String::new(scope, "webkitRequestFileSystem").unwrap();
        global.define_own_property(scope, name_webkit_request_file_system.into(), ctor_webkit_request_file_system.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_webkit_resolve_local_file_system_url) = tmpl_webkit_resolve_local_file_system_url.get_function(scope) {
        let name_webkit_resolve_local_file_system_url = v8::String::new(scope, "webkitResolveLocalFileSystemURL").unwrap();
        global.define_own_property(scope, name_webkit_resolve_local_file_system_url.into(), ctor_webkit_resolve_local_file_system_url.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_webkit_speech_grammar_list) = tmpl_webkit_speech_grammar_list.get_function(scope) {
        let name_webkit_speech_grammar_list = v8::String::new(scope, "webkitSpeechGrammarList").unwrap();
        global.define_own_property(scope, name_webkit_speech_grammar_list.into(), ctor_webkit_speech_grammar_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_webkit_speech_recognition) = tmpl_webkit_speech_recognition.get_function(scope) {
        let name_webkit_speech_recognition = v8::String::new(scope, "webkitSpeechRecognition").unwrap();
        global.define_own_property(scope, name_webkit_speech_recognition.into(), ctor_webkit_speech_recognition.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_webkit_speech_recognition_error) = tmpl_webkit_speech_recognition_error.get_function(scope) {
        let name_webkit_speech_recognition_error = v8::String::new(scope, "webkitSpeechRecognitionError").unwrap();
        global.define_own_property(scope, name_webkit_speech_recognition_error.into(), ctor_webkit_speech_recognition_error.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_webkit_speech_recognition_event) = tmpl_webkit_speech_recognition_event.get_function(scope) {
        let name_webkit_speech_recognition_event = v8::String::new(scope, "webkitSpeechRecognitionEvent").unwrap();
        global.define_own_property(scope, name_webkit_speech_recognition_event.into(), ctor_webkit_speech_recognition_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_webkit_storage_info) = tmpl_webkit_storage_info.get_function(scope) {
        let name_webkit_storage_info = v8::String::new(scope, "webkitStorageInfo").unwrap();
        global.define_own_property(scope, name_webkit_storage_info.into(), ctor_webkit_storage_info.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_webkit_url) = tmpl_webkit_url.get_function(scope) {
        let name_webkit_url = v8::String::new(scope, "webkitURL").unwrap();
        global.define_own_property(scope, name_webkit_url.into(), ctor_webkit_url.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_task_controller) = tmpl_task_controller.get_function(scope) {
        let name_task_controller = v8::String::new(scope, "TaskController").unwrap();
        global.define_own_property(scope, name_task_controller.into(), ctor_task_controller.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_range) = tmpl_range.get_function(scope) {
        let name_range = v8::String::new(scope, "Range").unwrap();
        global.define_own_property(scope, name_range.into(), ctor_range.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_static_range) = tmpl_static_range.get_function(scope) {
        let name_static_range = v8::String::new(scope, "StaticRange").unwrap();
        global.define_own_property(scope, name_static_range.into(), ctor_static_range.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_keyframe_effect) = tmpl_keyframe_effect.get_function(scope) {
        let name_keyframe_effect = v8::String::new(scope, "KeyframeEffect").unwrap();
        global.define_own_property(scope, name_keyframe_effect.into(), ctor_keyframe_effect.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_document_timeline) = tmpl_document_timeline.get_function(scope) {
        let name_document_timeline = v8::String::new(scope, "DocumentTimeline").unwrap();
        global.define_own_property(scope, name_document_timeline.into(), ctor_document_timeline.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_pointer_timeline) = tmpl_pointer_timeline.get_function(scope) {
        let name_pointer_timeline = v8::String::new(scope, "PointerTimeline").unwrap();
        global.define_own_property(scope, name_pointer_timeline.into(), ctor_pointer_timeline.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_scroll_timeline) = tmpl_scroll_timeline.get_function(scope) {
        let name_scroll_timeline = v8::String::new(scope, "ScrollTimeline").unwrap();
        global.define_own_property(scope, name_scroll_timeline.into(), ctor_scroll_timeline.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_authenticator_assertion_response) = tmpl_authenticator_assertion_response.get_function(scope) {
        let name_authenticator_assertion_response = v8::String::new(scope, "AuthenticatorAssertionResponse").unwrap();
        global.define_own_property(scope, name_authenticator_assertion_response.into(), ctor_authenticator_assertion_response.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_authenticator_attestation_response) = tmpl_authenticator_attestation_response.get_function(scope) {
        let name_authenticator_attestation_response = v8::String::new(scope, "AuthenticatorAttestationResponse").unwrap();
        global.define_own_property(scope, name_authenticator_attestation_response.into(), ctor_authenticator_attestation_response.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_file) = tmpl_file.get_function(scope) {
        let name_file = v8::String::new(scope, "File").unwrap();
        global.define_own_property(scope, name_file.into(), ctor_file.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssparser_at_rule) = tmpl_cssparser_at_rule.get_function(scope) {
        let name_cssparser_at_rule = v8::String::new(scope, "CSSParserAtRule").unwrap();
        global.define_own_property(scope, name_cssparser_at_rule.into(), ctor_cssparser_at_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssparser_declaration) = tmpl_cssparser_declaration.get_function(scope) {
        let name_cssparser_declaration = v8::String::new(scope, "CSSParserDeclaration").unwrap();
        global.define_own_property(scope, name_cssparser_declaration.into(), ctor_cssparser_declaration.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssparser_qualified_rule) = tmpl_cssparser_qualified_rule.get_function(scope) {
        let name_cssparser_qualified_rule = v8::String::new(scope, "CSSParserQualifiedRule").unwrap();
        global.define_own_property(scope, name_cssparser_qualified_rule.into(), ctor_cssparser_qualified_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssparser_block) = tmpl_cssparser_block.get_function(scope) {
        let name_cssparser_block = v8::String::new(scope, "CSSParserBlock").unwrap();
        global.define_own_property(scope, name_cssparser_block.into(), ctor_cssparser_block.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssparser_function) = tmpl_cssparser_function.get_function(scope) {
        let name_cssparser_function = v8::String::new(scope, "CSSParserFunction").unwrap();
        global.define_own_property(scope, name_cssparser_function.into(), ctor_cssparser_function.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssapply_statement_rule) = tmpl_cssapply_statement_rule.get_function(scope) {
        let name_cssapply_statement_rule = v8::String::new(scope, "CSSApplyStatementRule").unwrap();
        global.define_own_property(scope, name_cssapply_statement_rule.into(), ctor_cssapply_statement_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csscolor_profile_rule) = tmpl_csscolor_profile_rule.get_function(scope) {
        let name_csscolor_profile_rule = v8::String::new(scope, "CSSColorProfileRule").unwrap();
        global.define_own_property(scope, name_csscolor_profile_rule.into(), ctor_csscolor_profile_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csscontents_statement_rule) = tmpl_csscontents_statement_rule.get_function(scope) {
        let name_csscontents_statement_rule = v8::String::new(scope, "CSSContentsStatementRule").unwrap();
        global.define_own_property(scope, name_csscontents_statement_rule.into(), ctor_csscontents_statement_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csscounter_style_rule) = tmpl_csscounter_style_rule.get_function(scope) {
        let name_csscounter_style_rule = v8::String::new(scope, "CSSCounterStyleRule").unwrap();
        global.define_own_property(scope, name_csscounter_style_rule.into(), ctor_csscounter_style_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csscustom_media_rule) = tmpl_csscustom_media_rule.get_function(scope) {
        let name_csscustom_media_rule = v8::String::new(scope, "CSSCustomMediaRule").unwrap();
        global.define_own_property(scope, name_csscustom_media_rule.into(), ctor_csscustom_media_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssfont_face_rule) = tmpl_cssfont_face_rule.get_function(scope) {
        let name_cssfont_face_rule = v8::String::new(scope, "CSSFontFaceRule").unwrap();
        global.define_own_property(scope, name_cssfont_face_rule.into(), ctor_cssfont_face_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssfont_feature_values_rule) = tmpl_cssfont_feature_values_rule.get_function(scope) {
        let name_cssfont_feature_values_rule = v8::String::new(scope, "CSSFontFeatureValuesRule").unwrap();
        global.define_own_property(scope, name_cssfont_feature_values_rule.into(), ctor_cssfont_feature_values_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssfont_palette_values_rule) = tmpl_cssfont_palette_values_rule.get_function(scope) {
        let name_cssfont_palette_values_rule = v8::String::new(scope, "CSSFontPaletteValuesRule").unwrap();
        global.define_own_property(scope, name_cssfont_palette_values_rule.into(), ctor_cssfont_palette_values_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssfunction_declarations) = tmpl_cssfunction_declarations.get_function(scope) {
        let name_cssfunction_declarations = v8::String::new(scope, "CSSFunctionDeclarations").unwrap();
        global.define_own_property(scope, name_cssfunction_declarations.into(), ctor_cssfunction_declarations.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssgrouping_rule) = tmpl_cssgrouping_rule.get_function(scope) {
        let name_cssgrouping_rule = v8::String::new(scope, "CSSGroupingRule").unwrap();
        global.define_own_property(scope, name_cssgrouping_rule.into(), ctor_cssgrouping_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssimport_rule) = tmpl_cssimport_rule.get_function(scope) {
        let name_cssimport_rule = v8::String::new(scope, "CSSImportRule").unwrap();
        global.define_own_property(scope, name_cssimport_rule.into(), ctor_cssimport_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csskeyframe_rule) = tmpl_csskeyframe_rule.get_function(scope) {
        let name_csskeyframe_rule = v8::String::new(scope, "CSSKeyframeRule").unwrap();
        global.define_own_property(scope, name_csskeyframe_rule.into(), ctor_csskeyframe_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csskeyframes_rule) = tmpl_csskeyframes_rule.get_function(scope) {
        let name_csskeyframes_rule = v8::String::new(scope, "CSSKeyframesRule").unwrap();
        global.define_own_property(scope, name_csskeyframes_rule.into(), ctor_csskeyframes_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csslayer_statement_rule) = tmpl_csslayer_statement_rule.get_function(scope) {
        let name_csslayer_statement_rule = v8::String::new(scope, "CSSLayerStatementRule").unwrap();
        global.define_own_property(scope, name_csslayer_statement_rule.into(), ctor_csslayer_statement_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssmargin_rule) = tmpl_cssmargin_rule.get_function(scope) {
        let name_cssmargin_rule = v8::String::new(scope, "CSSMarginRule").unwrap();
        global.define_own_property(scope, name_cssmargin_rule.into(), ctor_cssmargin_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssnamespace_rule) = tmpl_cssnamespace_rule.get_function(scope) {
        let name_cssnamespace_rule = v8::String::new(scope, "CSSNamespaceRule").unwrap();
        global.define_own_property(scope, name_cssnamespace_rule.into(), ctor_cssnamespace_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssnested_declarations) = tmpl_cssnested_declarations.get_function(scope) {
        let name_cssnested_declarations = v8::String::new(scope, "CSSNestedDeclarations").unwrap();
        global.define_own_property(scope, name_cssnested_declarations.into(), ctor_cssnested_declarations.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssposition_try_rule) = tmpl_cssposition_try_rule.get_function(scope) {
        let name_cssposition_try_rule = v8::String::new(scope, "CSSPositionTryRule").unwrap();
        global.define_own_property(scope, name_cssposition_try_rule.into(), ctor_cssposition_try_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssproperty_rule) = tmpl_cssproperty_rule.get_function(scope) {
        let name_cssproperty_rule = v8::String::new(scope, "CSSPropertyRule").unwrap();
        global.define_own_property(scope, name_cssproperty_rule.into(), ctor_cssproperty_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssview_transition_rule) = tmpl_cssview_transition_rule.get_function(scope) {
        let name_cssview_transition_rule = v8::String::new(scope, "CSSViewTransitionRule").unwrap();
        global.define_own_property(scope, name_cssview_transition_rule.into(), ctor_cssview_transition_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssfont_face_descriptors) = tmpl_cssfont_face_descriptors.get_function(scope) {
        let name_cssfont_face_descriptors = v8::String::new(scope, "CSSFontFaceDescriptors").unwrap();
        global.define_own_property(scope, name_cssfont_face_descriptors.into(), ctor_cssfont_face_descriptors.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssfunction_descriptors) = tmpl_cssfunction_descriptors.get_function(scope) {
        let name_cssfunction_descriptors = v8::String::new(scope, "CSSFunctionDescriptors").unwrap();
        global.define_own_property(scope, name_cssfunction_descriptors.into(), ctor_cssfunction_descriptors.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csspage_descriptors) = tmpl_csspage_descriptors.get_function(scope) {
        let name_csspage_descriptors = v8::String::new(scope, "CSSPageDescriptors").unwrap();
        global.define_own_property(scope, name_csspage_descriptors.into(), ctor_csspage_descriptors.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssposition_try_descriptors) = tmpl_cssposition_try_descriptors.get_function(scope) {
        let name_cssposition_try_descriptors = v8::String::new(scope, "CSSPositionTryDescriptors").unwrap();
        global.define_own_property(scope, name_cssposition_try_descriptors.into(), ctor_cssposition_try_descriptors.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssstyle_properties) = tmpl_cssstyle_properties.get_function(scope) {
        let name_cssstyle_properties = v8::String::new(scope, "CSSStyleProperties").unwrap();
        global.define_own_property(scope, name_cssstyle_properties.into(), ctor_cssstyle_properties.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csscolor_value) = tmpl_csscolor_value.get_function(scope) {
        let name_csscolor_value = v8::String::new(scope, "CSSColorValue").unwrap();
        global.define_own_property(scope, name_csscolor_value.into(), ctor_csscolor_value.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssimage_value) = tmpl_cssimage_value.get_function(scope) {
        let name_cssimage_value = v8::String::new(scope, "CSSImageValue").unwrap();
        global.define_own_property(scope, name_cssimage_value.into(), ctor_cssimage_value.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csskeyword_value) = tmpl_csskeyword_value.get_function(scope) {
        let name_csskeyword_value = v8::String::new(scope, "CSSKeywordValue").unwrap();
        global.define_own_property(scope, name_csskeyword_value.into(), ctor_csskeyword_value.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssnumeric_value) = tmpl_cssnumeric_value.get_function(scope) {
        let name_cssnumeric_value = v8::String::new(scope, "CSSNumericValue").unwrap();
        global.define_own_property(scope, name_cssnumeric_value.into(), ctor_cssnumeric_value.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csstransform_value) = tmpl_csstransform_value.get_function(scope) {
        let name_csstransform_value = v8::String::new(scope, "CSSTransformValue").unwrap();
        global.define_own_property(scope, name_csstransform_value.into(), ctor_csstransform_value.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssunparsed_value) = tmpl_cssunparsed_value.get_function(scope) {
        let name_cssunparsed_value = v8::String::new(scope, "CSSUnparsedValue").unwrap();
        global.define_own_property(scope, name_cssunparsed_value.into(), ctor_cssunparsed_value.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssmatrix_component) = tmpl_cssmatrix_component.get_function(scope) {
        let name_cssmatrix_component = v8::String::new(scope, "CSSMatrixComponent").unwrap();
        global.define_own_property(scope, name_cssmatrix_component.into(), ctor_cssmatrix_component.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssperspective) = tmpl_cssperspective.get_function(scope) {
        let name_cssperspective = v8::String::new(scope, "CSSPerspective").unwrap();
        global.define_own_property(scope, name_cssperspective.into(), ctor_cssperspective.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssrotate) = tmpl_cssrotate.get_function(scope) {
        let name_cssrotate = v8::String::new(scope, "CSSRotate").unwrap();
        global.define_own_property(scope, name_cssrotate.into(), ctor_cssrotate.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssscale) = tmpl_cssscale.get_function(scope) {
        let name_cssscale = v8::String::new(scope, "CSSScale").unwrap();
        global.define_own_property(scope, name_cssscale.into(), ctor_cssscale.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssskew) = tmpl_cssskew.get_function(scope) {
        let name_cssskew = v8::String::new(scope, "CSSSkew").unwrap();
        global.define_own_property(scope, name_cssskew.into(), ctor_cssskew.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssskew_x) = tmpl_cssskew_x.get_function(scope) {
        let name_cssskew_x = v8::String::new(scope, "CSSSkewX").unwrap();
        global.define_own_property(scope, name_cssskew_x.into(), ctor_cssskew_x.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssskew_y) = tmpl_cssskew_y.get_function(scope) {
        let name_cssskew_y = v8::String::new(scope, "CSSSkewY").unwrap();
        global.define_own_property(scope, name_cssskew_y.into(), ctor_cssskew_y.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csstranslate) = tmpl_csstranslate.get_function(scope) {
        let name_csstranslate = v8::String::new(scope, "CSSTranslate").unwrap();
        global.define_own_property(scope, name_csstranslate.into(), ctor_csstranslate.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_window_client) = tmpl_window_client.get_function(scope) {
        let name_window_client = v8::String::new(scope, "WindowClient").unwrap();
        global.define_own_property(scope, name_window_client.into(), ctor_window_client.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_digital_credential) = tmpl_digital_credential.get_function(scope) {
        let name_digital_credential = v8::String::new(scope, "DigitalCredential").unwrap();
        global.define_own_property(scope, name_digital_credential.into(), ctor_digital_credential.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_federated_credential) = tmpl_federated_credential.get_function(scope) {
        let name_federated_credential = v8::String::new(scope, "FederatedCredential").unwrap();
        global.define_own_property(scope, name_federated_credential.into(), ctor_federated_credential.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_identity_credential) = tmpl_identity_credential.get_function(scope) {
        let name_identity_credential = v8::String::new(scope, "IdentityCredential").unwrap();
        global.define_own_property(scope, name_identity_credential.into(), ctor_identity_credential.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_otpcredential) = tmpl_otpcredential.get_function(scope) {
        let name_otpcredential = v8::String::new(scope, "OTPCredential").unwrap();
        global.define_own_property(scope, name_otpcredential.into(), ctor_otpcredential.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_password_credential) = tmpl_password_credential.get_function(scope) {
        let name_password_credential = v8::String::new(scope, "PasswordCredential").unwrap();
        global.define_own_property(scope, name_password_credential.into(), ctor_password_credential.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_public_key_credential) = tmpl_public_key_credential.get_function(scope) {
        let name_public_key_credential = v8::String::new(scope, "PublicKeyCredential").unwrap();
        global.define_own_property(scope, name_public_key_credential.into(), ctor_public_key_credential.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpupipeline_error) = tmpl_gpupipeline_error.get_function(scope) {
        let name_gpupipeline_error = v8::String::new(scope, "GPUPipelineError").unwrap();
        global.define_own_property(scope, name_gpupipeline_error.into(), ctor_gpupipeline_error.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_identity_credential_error) = tmpl_identity_credential_error.get_function(scope) {
        let name_identity_credential_error = v8::String::new(scope, "IdentityCredentialError").unwrap();
        global.define_own_property(scope, name_identity_credential_error.into(), ctor_identity_credential_error.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_overconstrained_error) = tmpl_overconstrained_error.get_function(scope) {
        let name_overconstrained_error = v8::String::new(scope, "OverconstrainedError").unwrap();
        global.define_own_property(scope, name_overconstrained_error.into(), ctor_overconstrained_error.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_quota_exceeded_error) = tmpl_quota_exceeded_error.get_function(scope) {
        let name_quota_exceeded_error = v8::String::new(scope, "QuotaExceededError").unwrap();
        global.define_own_property(scope, name_quota_exceeded_error.into(), ctor_quota_exceeded_error.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcerror) = tmpl_rtcerror.get_function(scope) {
        let name_rtcerror = v8::String::new(scope, "RTCError").unwrap();
        global.define_own_property(scope, name_rtcerror.into(), ctor_rtcerror.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_transport_error) = tmpl_web_transport_error.get_function(scope) {
        let name_web_transport_error = v8::String::new(scope, "WebTransportError").unwrap();
        global.define_own_property(scope, name_web_transport_error.into(), ctor_web_transport_error.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_dommatrix) = tmpl_dommatrix.get_function(scope) {
        let name_dommatrix = v8::String::new(scope, "DOMMatrix").unwrap();
        global.define_own_property(scope, name_dommatrix.into(), ctor_dommatrix.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_dompoint) = tmpl_dompoint.get_function(scope) {
        let name_dompoint = v8::String::new(scope, "DOMPoint").unwrap();
        global.define_own_property(scope, name_dompoint.into(), ctor_dompoint.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_domrect) = tmpl_domrect.get_function(scope) {
        let name_domrect = v8::String::new(scope, "DOMRect").unwrap();
        global.define_own_property(scope, name_domrect.into(), ctor_domrect.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_animation_event) = tmpl_animation_event.get_function(scope) {
        let name_animation_event = v8::String::new(scope, "AnimationEvent").unwrap();
        global.define_own_property(scope, name_animation_event.into(), ctor_animation_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_animation_playback_event) = tmpl_animation_playback_event.get_function(scope) {
        let name_animation_playback_event = v8::String::new(scope, "AnimationPlaybackEvent").unwrap();
        global.define_own_property(scope, name_animation_playback_event.into(), ctor_animation_playback_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_audio_processing_event) = tmpl_audio_processing_event.get_function(scope) {
        let name_audio_processing_event = v8::String::new(scope, "AudioProcessingEvent").unwrap();
        global.define_own_property(scope, name_audio_processing_event.into(), ctor_audio_processing_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_autofill_event) = tmpl_autofill_event.get_function(scope) {
        let name_autofill_event = v8::String::new(scope, "AutofillEvent").unwrap();
        global.define_own_property(scope, name_autofill_event.into(), ctor_autofill_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_before_install_prompt_event) = tmpl_before_install_prompt_event.get_function(scope) {
        let name_before_install_prompt_event = v8::String::new(scope, "BeforeInstallPromptEvent").unwrap();
        global.define_own_property(scope, name_before_install_prompt_event.into(), ctor_before_install_prompt_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_before_unload_event) = tmpl_before_unload_event.get_function(scope) {
        let name_before_unload_event = v8::String::new(scope, "BeforeUnloadEvent").unwrap();
        global.define_own_property(scope, name_before_unload_event.into(), ctor_before_unload_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_blob_event) = tmpl_blob_event.get_function(scope) {
        let name_blob_event = v8::String::new(scope, "BlobEvent").unwrap();
        global.define_own_property(scope, name_blob_event.into(), ctor_blob_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_advertising_event) = tmpl_bluetooth_advertising_event.get_function(scope) {
        let name_bluetooth_advertising_event = v8::String::new(scope, "BluetoothAdvertisingEvent").unwrap();
        global.define_own_property(scope, name_bluetooth_advertising_event.into(), ctor_bluetooth_advertising_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_buffered_change_event) = tmpl_buffered_change_event.get_function(scope) {
        let name_buffered_change_event = v8::String::new(scope, "BufferedChangeEvent").unwrap();
        global.define_own_property(scope, name_buffered_change_event.into(), ctor_buffered_change_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_capture_action_event) = tmpl_capture_action_event.get_function(scope) {
        let name_capture_action_event = v8::String::new(scope, "CaptureActionEvent").unwrap();
        global.define_own_property(scope, name_capture_action_event.into(), ctor_capture_action_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_captured_mouse_event) = tmpl_captured_mouse_event.get_function(scope) {
        let name_captured_mouse_event = v8::String::new(scope, "CapturedMouseEvent").unwrap();
        global.define_own_property(scope, name_captured_mouse_event.into(), ctor_captured_mouse_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_character_bounds_update_event) = tmpl_character_bounds_update_event.get_function(scope) {
        let name_character_bounds_update_event = v8::String::new(scope, "CharacterBoundsUpdateEvent").unwrap();
        global.define_own_property(scope, name_character_bounds_update_event.into(), ctor_character_bounds_update_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_clipboard_change_event) = tmpl_clipboard_change_event.get_function(scope) {
        let name_clipboard_change_event = v8::String::new(scope, "ClipboardChangeEvent").unwrap();
        global.define_own_property(scope, name_clipboard_change_event.into(), ctor_clipboard_change_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_clipboard_event) = tmpl_clipboard_event.get_function(scope) {
        let name_clipboard_event = v8::String::new(scope, "ClipboardEvent").unwrap();
        global.define_own_property(scope, name_clipboard_event.into(), ctor_clipboard_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_close_event) = tmpl_close_event.get_function(scope) {
        let name_close_event = v8::String::new(scope, "CloseEvent").unwrap();
        global.define_own_property(scope, name_close_event.into(), ctor_close_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_command_event) = tmpl_command_event.get_function(scope) {
        let name_command_event = v8::String::new(scope, "CommandEvent").unwrap();
        global.define_own_property(scope, name_command_event.into(), ctor_command_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_content_visibility_auto_state_change_event) = tmpl_content_visibility_auto_state_change_event.get_function(scope) {
        let name_content_visibility_auto_state_change_event = v8::String::new(scope, "ContentVisibilityAutoStateChangeEvent").unwrap();
        global.define_own_property(scope, name_content_visibility_auto_state_change_event.into(), ctor_content_visibility_auto_state_change_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cookie_change_event) = tmpl_cookie_change_event.get_function(scope) {
        let name_cookie_change_event = v8::String::new(scope, "CookieChangeEvent").unwrap();
        global.define_own_property(scope, name_cookie_change_event.into(), ctor_cookie_change_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_custom_event) = tmpl_custom_event.get_function(scope) {
        let name_custom_event = v8::String::new(scope, "CustomEvent").unwrap();
        global.define_own_property(scope, name_custom_event.into(), ctor_custom_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_device_change_event) = tmpl_device_change_event.get_function(scope) {
        let name_device_change_event = v8::String::new(scope, "DeviceChangeEvent").unwrap();
        global.define_own_property(scope, name_device_change_event.into(), ctor_device_change_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_device_motion_event) = tmpl_device_motion_event.get_function(scope) {
        let name_device_motion_event = v8::String::new(scope, "DeviceMotionEvent").unwrap();
        global.define_own_property(scope, name_device_motion_event.into(), ctor_device_motion_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_device_orientation_event) = tmpl_device_orientation_event.get_function(scope) {
        let name_device_orientation_event = v8::String::new(scope, "DeviceOrientationEvent").unwrap();
        global.define_own_property(scope, name_device_orientation_event.into(), ctor_device_orientation_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_document_picture_in_picture_event) = tmpl_document_picture_in_picture_event.get_function(scope) {
        let name_document_picture_in_picture_event = v8::String::new(scope, "DocumentPictureInPictureEvent").unwrap();
        global.define_own_property(scope, name_document_picture_in_picture_event.into(), ctor_document_picture_in_picture_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_error_event) = tmpl_error_event.get_function(scope) {
        let name_error_event = v8::String::new(scope, "ErrorEvent").unwrap();
        global.define_own_property(scope, name_error_event.into(), ctor_error_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_extendable_event) = tmpl_extendable_event.get_function(scope) {
        let name_extendable_event = v8::String::new(scope, "ExtendableEvent").unwrap();
        global.define_own_property(scope, name_extendable_event.into(), ctor_extendable_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_font_face_set_load_event) = tmpl_font_face_set_load_event.get_function(scope) {
        let name_font_face_set_load_event = v8::String::new(scope, "FontFaceSetLoadEvent").unwrap();
        global.define_own_property(scope, name_font_face_set_load_event.into(), ctor_font_face_set_load_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_form_data_event) = tmpl_form_data_event.get_function(scope) {
        let name_form_data_event = v8::String::new(scope, "FormDataEvent").unwrap();
        global.define_own_property(scope, name_form_data_event.into(), ctor_form_data_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpuuncaptured_error_event) = tmpl_gpuuncaptured_error_event.get_function(scope) {
        let name_gpuuncaptured_error_event = v8::String::new(scope, "GPUUncapturedErrorEvent").unwrap();
        global.define_own_property(scope, name_gpuuncaptured_error_event.into(), ctor_gpuuncaptured_error_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gamepad_event) = tmpl_gamepad_event.get_function(scope) {
        let name_gamepad_event = v8::String::new(scope, "GamepadEvent").unwrap();
        global.define_own_property(scope, name_gamepad_event.into(), ctor_gamepad_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_hidconnection_event) = tmpl_hidconnection_event.get_function(scope) {
        let name_hidconnection_event = v8::String::new(scope, "HIDConnectionEvent").unwrap();
        global.define_own_property(scope, name_hidconnection_event.into(), ctor_hidconnection_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_hidinput_report_event) = tmpl_hidinput_report_event.get_function(scope) {
        let name_hidinput_report_event = v8::String::new(scope, "HIDInputReportEvent").unwrap();
        global.define_own_property(scope, name_hidinput_report_event.into(), ctor_hidinput_report_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_hash_change_event) = tmpl_hash_change_event.get_function(scope) {
        let name_hash_change_event = v8::String::new(scope, "HashChangeEvent").unwrap();
        global.define_own_property(scope, name_hash_change_event.into(), ctor_hash_change_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idbversion_change_event) = tmpl_idbversion_change_event.get_function(scope) {
        let name_idbversion_change_event = v8::String::new(scope, "IDBVersionChangeEvent").unwrap();
        global.define_own_property(scope, name_idbversion_change_event.into(), ctor_idbversion_change_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_key_frame_request_event) = tmpl_key_frame_request_event.get_function(scope) {
        let name_key_frame_request_event = v8::String::new(scope, "KeyFrameRequestEvent").unwrap();
        global.define_own_property(scope, name_key_frame_request_event.into(), ctor_key_frame_request_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_midiconnection_event) = tmpl_midiconnection_event.get_function(scope) {
        let name_midiconnection_event = v8::String::new(scope, "MIDIConnectionEvent").unwrap();
        global.define_own_property(scope, name_midiconnection_event.into(), ctor_midiconnection_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_midimessage_event) = tmpl_midimessage_event.get_function(scope) {
        let name_midimessage_event = v8::String::new(scope, "MIDIMessageEvent").unwrap();
        global.define_own_property(scope, name_midimessage_event.into(), ctor_midimessage_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_encrypted_event) = tmpl_media_encrypted_event.get_function(scope) {
        let name_media_encrypted_event = v8::String::new(scope, "MediaEncryptedEvent").unwrap();
        global.define_own_property(scope, name_media_encrypted_event.into(), ctor_media_encrypted_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_key_message_event) = tmpl_media_key_message_event.get_function(scope) {
        let name_media_key_message_event = v8::String::new(scope, "MediaKeyMessageEvent").unwrap();
        global.define_own_property(scope, name_media_key_message_event.into(), ctor_media_key_message_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_query_list_event) = tmpl_media_query_list_event.get_function(scope) {
        let name_media_query_list_event = v8::String::new(scope, "MediaQueryListEvent").unwrap();
        global.define_own_property(scope, name_media_query_list_event.into(), ctor_media_query_list_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_stream_track_event) = tmpl_media_stream_track_event.get_function(scope) {
        let name_media_stream_track_event = v8::String::new(scope, "MediaStreamTrackEvent").unwrap();
        global.define_own_property(scope, name_media_stream_track_event.into(), ctor_media_stream_track_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_message_event) = tmpl_message_event.get_function(scope) {
        let name_message_event = v8::String::new(scope, "MessageEvent").unwrap();
        global.define_own_property(scope, name_message_event.into(), ctor_message_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_ndefreading_event) = tmpl_ndefreading_event.get_function(scope) {
        let name_ndefreading_event = v8::String::new(scope, "NDEFReadingEvent").unwrap();
        global.define_own_property(scope, name_ndefreading_event.into(), ctor_ndefreading_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_navigate_event) = tmpl_navigate_event.get_function(scope) {
        let name_navigate_event = v8::String::new(scope, "NavigateEvent").unwrap();
        global.define_own_property(scope, name_navigate_event.into(), ctor_navigate_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_navigation_current_entry_change_event) = tmpl_navigation_current_entry_change_event.get_function(scope) {
        let name_navigation_current_entry_change_event = v8::String::new(scope, "NavigationCurrentEntryChangeEvent").unwrap();
        global.define_own_property(scope, name_navigation_current_entry_change_event.into(), ctor_navigation_current_entry_change_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_offline_audio_completion_event) = tmpl_offline_audio_completion_event.get_function(scope) {
        let name_offline_audio_completion_event = v8::String::new(scope, "OfflineAudioCompletionEvent").unwrap();
        global.define_own_property(scope, name_offline_audio_completion_event.into(), ctor_offline_audio_completion_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_page_reveal_event) = tmpl_page_reveal_event.get_function(scope) {
        let name_page_reveal_event = v8::String::new(scope, "PageRevealEvent").unwrap();
        global.define_own_property(scope, name_page_reveal_event.into(), ctor_page_reveal_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_page_swap_event) = tmpl_page_swap_event.get_function(scope) {
        let name_page_swap_event = v8::String::new(scope, "PageSwapEvent").unwrap();
        global.define_own_property(scope, name_page_swap_event.into(), ctor_page_swap_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_page_transition_event) = tmpl_page_transition_event.get_function(scope) {
        let name_page_transition_event = v8::String::new(scope, "PageTransitionEvent").unwrap();
        global.define_own_property(scope, name_page_transition_event.into(), ctor_page_transition_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_payment_request_update_event) = tmpl_payment_request_update_event.get_function(scope) {
        let name_payment_request_update_event = v8::String::new(scope, "PaymentRequestUpdateEvent").unwrap();
        global.define_own_property(scope, name_payment_request_update_event.into(), ctor_payment_request_update_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_picture_in_picture_event) = tmpl_picture_in_picture_event.get_function(scope) {
        let name_picture_in_picture_event = v8::String::new(scope, "PictureInPictureEvent").unwrap();
        global.define_own_property(scope, name_picture_in_picture_event.into(), ctor_picture_in_picture_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_pop_state_event) = tmpl_pop_state_event.get_function(scope) {
        let name_pop_state_event = v8::String::new(scope, "PopStateEvent").unwrap();
        global.define_own_property(scope, name_pop_state_event.into(), ctor_pop_state_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_portal_activate_event) = tmpl_portal_activate_event.get_function(scope) {
        let name_portal_activate_event = v8::String::new(scope, "PortalActivateEvent").unwrap();
        global.define_own_property(scope, name_portal_activate_event.into(), ctor_portal_activate_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_presentation_connection_available_event) = tmpl_presentation_connection_available_event.get_function(scope) {
        let name_presentation_connection_available_event = v8::String::new(scope, "PresentationConnectionAvailableEvent").unwrap();
        global.define_own_property(scope, name_presentation_connection_available_event.into(), ctor_presentation_connection_available_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_presentation_connection_close_event) = tmpl_presentation_connection_close_event.get_function(scope) {
        let name_presentation_connection_close_event = v8::String::new(scope, "PresentationConnectionCloseEvent").unwrap();
        global.define_own_property(scope, name_presentation_connection_close_event.into(), ctor_presentation_connection_close_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_progress_event) = tmpl_progress_event.get_function(scope) {
        let name_progress_event = v8::String::new(scope, "ProgressEvent").unwrap();
        global.define_own_property(scope, name_progress_event.into(), ctor_progress_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_promise_rejection_event) = tmpl_promise_rejection_event.get_function(scope) {
        let name_promise_rejection_event = v8::String::new(scope, "PromiseRejectionEvent").unwrap();
        global.define_own_property(scope, name_promise_rejection_event.into(), ctor_promise_rejection_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcdtmftone_change_event) = tmpl_rtcdtmftone_change_event.get_function(scope) {
        let name_rtcdtmftone_change_event = v8::String::new(scope, "RTCDTMFToneChangeEvent").unwrap();
        global.define_own_property(scope, name_rtcdtmftone_change_event.into(), ctor_rtcdtmftone_change_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcdata_channel_event) = tmpl_rtcdata_channel_event.get_function(scope) {
        let name_rtcdata_channel_event = v8::String::new(scope, "RTCDataChannelEvent").unwrap();
        global.define_own_property(scope, name_rtcdata_channel_event.into(), ctor_rtcdata_channel_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcerror_event) = tmpl_rtcerror_event.get_function(scope) {
        let name_rtcerror_event = v8::String::new(scope, "RTCErrorEvent").unwrap();
        global.define_own_property(scope, name_rtcerror_event.into(), ctor_rtcerror_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcpeer_connection_ice_error_event) = tmpl_rtcpeer_connection_ice_error_event.get_function(scope) {
        let name_rtcpeer_connection_ice_error_event = v8::String::new(scope, "RTCPeerConnectionIceErrorEvent").unwrap();
        global.define_own_property(scope, name_rtcpeer_connection_ice_error_event.into(), ctor_rtcpeer_connection_ice_error_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcpeer_connection_ice_event) = tmpl_rtcpeer_connection_ice_event.get_function(scope) {
        let name_rtcpeer_connection_ice_event = v8::String::new(scope, "RTCPeerConnectionIceEvent").unwrap();
        global.define_own_property(scope, name_rtcpeer_connection_ice_event.into(), ctor_rtcpeer_connection_ice_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtctrack_event) = tmpl_rtctrack_event.get_function(scope) {
        let name_rtctrack_event = v8::String::new(scope, "RTCTrackEvent").unwrap();
        global.define_own_property(scope, name_rtctrack_event.into(), ctor_rtctrack_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtctransform_event) = tmpl_rtctransform_event.get_function(scope) {
        let name_rtctransform_event = v8::String::new(scope, "RTCTransformEvent").unwrap();
        global.define_own_property(scope, name_rtctransform_event.into(), ctor_rtctransform_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_sframe_transform_error_event) = tmpl_sframe_transform_error_event.get_function(scope) {
        let name_sframe_transform_error_event = v8::String::new(scope, "SFrameTransformErrorEvent").unwrap();
        global.define_own_property(scope, name_sframe_transform_error_event.into(), ctor_sframe_transform_error_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_security_policy_violation_event) = tmpl_security_policy_violation_event.get_function(scope) {
        let name_security_policy_violation_event = v8::String::new(scope, "SecurityPolicyViolationEvent").unwrap();
        global.define_own_property(scope, name_security_policy_violation_event.into(), ctor_security_policy_violation_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_sensor_error_event) = tmpl_sensor_error_event.get_function(scope) {
        let name_sensor_error_event = v8::String::new(scope, "SensorErrorEvent").unwrap();
        global.define_own_property(scope, name_sensor_error_event.into(), ctor_sensor_error_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_snap_event) = tmpl_snap_event.get_function(scope) {
        let name_snap_event = v8::String::new(scope, "SnapEvent").unwrap();
        global.define_own_property(scope, name_snap_event.into(), ctor_snap_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_speech_recognition_error_event) = tmpl_speech_recognition_error_event.get_function(scope) {
        let name_speech_recognition_error_event = v8::String::new(scope, "SpeechRecognitionErrorEvent").unwrap();
        global.define_own_property(scope, name_speech_recognition_error_event.into(), ctor_speech_recognition_error_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_speech_recognition_event) = tmpl_speech_recognition_event.get_function(scope) {
        let name_speech_recognition_event = v8::String::new(scope, "SpeechRecognitionEvent").unwrap();
        global.define_own_property(scope, name_speech_recognition_event.into(), ctor_speech_recognition_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_speech_synthesis_event) = tmpl_speech_synthesis_event.get_function(scope) {
        let name_speech_synthesis_event = v8::String::new(scope, "SpeechSynthesisEvent").unwrap();
        global.define_own_property(scope, name_speech_synthesis_event.into(), ctor_speech_synthesis_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_storage_event) = tmpl_storage_event.get_function(scope) {
        let name_storage_event = v8::String::new(scope, "StorageEvent").unwrap();
        global.define_own_property(scope, name_storage_event.into(), ctor_storage_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_submit_event) = tmpl_submit_event.get_function(scope) {
        let name_submit_event = v8::String::new(scope, "SubmitEvent").unwrap();
        global.define_own_property(scope, name_submit_event.into(), ctor_submit_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_task_priority_change_event) = tmpl_task_priority_change_event.get_function(scope) {
        let name_task_priority_change_event = v8::String::new(scope, "TaskPriorityChangeEvent").unwrap();
        global.define_own_property(scope, name_task_priority_change_event.into(), ctor_task_priority_change_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_text_format_update_event) = tmpl_text_format_update_event.get_function(scope) {
        let name_text_format_update_event = v8::String::new(scope, "TextFormatUpdateEvent").unwrap();
        global.define_own_property(scope, name_text_format_update_event.into(), ctor_text_format_update_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_text_update_event) = tmpl_text_update_event.get_function(scope) {
        let name_text_update_event = v8::String::new(scope, "TextUpdateEvent").unwrap();
        global.define_own_property(scope, name_text_update_event.into(), ctor_text_update_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_time_event) = tmpl_time_event.get_function(scope) {
        let name_time_event = v8::String::new(scope, "TimeEvent").unwrap();
        global.define_own_property(scope, name_time_event.into(), ctor_time_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_toggle_event) = tmpl_toggle_event.get_function(scope) {
        let name_toggle_event = v8::String::new(scope, "ToggleEvent").unwrap();
        global.define_own_property(scope, name_toggle_event.into(), ctor_toggle_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_track_event) = tmpl_track_event.get_function(scope) {
        let name_track_event = v8::String::new(scope, "TrackEvent").unwrap();
        global.define_own_property(scope, name_track_event.into(), ctor_track_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_transition_event) = tmpl_transition_event.get_function(scope) {
        let name_transition_event = v8::String::new(scope, "TransitionEvent").unwrap();
        global.define_own_property(scope, name_transition_event.into(), ctor_transition_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_uievent) = tmpl_uievent.get_function(scope) {
        let name_uievent = v8::String::new(scope, "UIEvent").unwrap();
        global.define_own_property(scope, name_uievent.into(), ctor_uievent.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usbconnection_event) = tmpl_usbconnection_event.get_function(scope) {
        let name_usbconnection_event = v8::String::new(scope, "USBConnectionEvent").unwrap();
        global.define_own_property(scope, name_usbconnection_event.into(), ctor_usbconnection_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_value_event) = tmpl_value_event.get_function(scope) {
        let name_value_event = v8::String::new(scope, "ValueEvent").unwrap();
        global.define_own_property(scope, name_value_event.into(), ctor_value_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_glcontext_event) = tmpl_web_glcontext_event.get_function(scope) {
        let name_web_glcontext_event = v8::String::new(scope, "WebGLContextEvent").unwrap();
        global.define_own_property(scope, name_web_glcontext_event.into(), ctor_web_glcontext_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_window_controls_overlay_geometry_change_event) = tmpl_window_controls_overlay_geometry_change_event.get_function(scope) {
        let name_window_controls_overlay_geometry_change_event = v8::String::new(scope, "WindowControlsOverlayGeometryChangeEvent").unwrap();
        global.define_own_property(scope, name_window_controls_overlay_geometry_change_event.into(), ctor_window_controls_overlay_geometry_change_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrinput_source_event) = tmpl_xrinput_source_event.get_function(scope) {
        let name_xrinput_source_event = v8::String::new(scope, "XRInputSourceEvent").unwrap();
        global.define_own_property(scope, name_xrinput_source_event.into(), ctor_xrinput_source_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrinput_sources_change_event) = tmpl_xrinput_sources_change_event.get_function(scope) {
        let name_xrinput_sources_change_event = v8::String::new(scope, "XRInputSourcesChangeEvent").unwrap();
        global.define_own_property(scope, name_xrinput_sources_change_event.into(), ctor_xrinput_sources_change_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrlayer_event) = tmpl_xrlayer_event.get_function(scope) {
        let name_xrlayer_event = v8::String::new(scope, "XRLayerEvent").unwrap();
        global.define_own_property(scope, name_xrlayer_event.into(), ctor_xrlayer_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrreference_space_event) = tmpl_xrreference_space_event.get_function(scope) {
        let name_xrreference_space_event = v8::String::new(scope, "XRReferenceSpaceEvent").unwrap();
        global.define_own_property(scope, name_xrreference_space_event.into(), ctor_xrreference_space_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrsession_event) = tmpl_xrsession_event.get_function(scope) {
        let name_xrsession_event = v8::String::new(scope, "XRSessionEvent").unwrap();
        global.define_own_property(scope, name_xrsession_event.into(), ctor_xrsession_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrvisibility_mask_change_event) = tmpl_xrvisibility_mask_change_event.get_function(scope) {
        let name_xrvisibility_mask_change_event = v8::String::new(scope, "XRVisibilityMaskChangeEvent").unwrap();
        global.define_own_property(scope, name_xrvisibility_mask_change_event.into(), ctor_xrvisibility_mask_change_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_abort_signal) = tmpl_abort_signal.get_function(scope) {
        let name_abort_signal = v8::String::new(scope, "AbortSignal").unwrap();
        global.define_own_property(scope, name_abort_signal.into(), ctor_abort_signal.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_animation) = tmpl_animation.get_function(scope) {
        let name_animation = v8::String::new(scope, "Animation").unwrap();
        global.define_own_property(scope, name_animation.into(), ctor_animation.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_audio_decoder) = tmpl_audio_decoder.get_function(scope) {
        let name_audio_decoder = v8::String::new(scope, "AudioDecoder").unwrap();
        global.define_own_property(scope, name_audio_decoder.into(), ctor_audio_decoder.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_audio_encoder) = tmpl_audio_encoder.get_function(scope) {
        let name_audio_encoder = v8::String::new(scope, "AudioEncoder").unwrap();
        global.define_own_property(scope, name_audio_encoder.into(), ctor_audio_encoder.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_audio_node) = tmpl_audio_node.get_function(scope) {
        let name_audio_node = v8::String::new(scope, "AudioNode").unwrap();
        global.define_own_property(scope, name_audio_node.into(), ctor_audio_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_audio_session) = tmpl_audio_session.get_function(scope) {
        let name_audio_session = v8::String::new(scope, "AudioSession").unwrap();
        global.define_own_property(scope, name_audio_session.into(), ctor_audio_session.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_audio_track_list) = tmpl_audio_track_list.get_function(scope) {
        let name_audio_track_list = v8::String::new(scope, "AudioTrackList").unwrap();
        global.define_own_property(scope, name_audio_track_list.into(), ctor_audio_track_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_background_fetch_registration) = tmpl_background_fetch_registration.get_function(scope) {
        let name_background_fetch_registration = v8::String::new(scope, "BackgroundFetchRegistration").unwrap();
        global.define_own_property(scope, name_background_fetch_registration.into(), ctor_background_fetch_registration.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_base_audio_context) = tmpl_base_audio_context.get_function(scope) {
        let name_base_audio_context = v8::String::new(scope, "BaseAudioContext").unwrap();
        global.define_own_property(scope, name_base_audio_context.into(), ctor_base_audio_context.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_battery_manager) = tmpl_battery_manager.get_function(scope) {
        let name_battery_manager = v8::String::new(scope, "BatteryManager").unwrap();
        global.define_own_property(scope, name_battery_manager.into(), ctor_battery_manager.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth) = tmpl_bluetooth.get_function(scope) {
        let name_bluetooth = v8::String::new(scope, "Bluetooth").unwrap();
        global.define_own_property(scope, name_bluetooth.into(), ctor_bluetooth.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_device) = tmpl_bluetooth_device.get_function(scope) {
        let name_bluetooth_device = v8::String::new(scope, "BluetoothDevice").unwrap();
        global.define_own_property(scope, name_bluetooth_device.into(), ctor_bluetooth_device.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_remote_gattcharacteristic) = tmpl_bluetooth_remote_gattcharacteristic.get_function(scope) {
        let name_bluetooth_remote_gattcharacteristic = v8::String::new(scope, "BluetoothRemoteGATTCharacteristic").unwrap();
        global.define_own_property(scope, name_bluetooth_remote_gattcharacteristic.into(), ctor_bluetooth_remote_gattcharacteristic.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_remote_gattservice) = tmpl_bluetooth_remote_gattservice.get_function(scope) {
        let name_bluetooth_remote_gattservice = v8::String::new(scope, "BluetoothRemoteGATTService").unwrap();
        global.define_own_property(scope, name_bluetooth_remote_gattservice.into(), ctor_bluetooth_remote_gattservice.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_broadcast_channel) = tmpl_broadcast_channel.get_function(scope) {
        let name_broadcast_channel = v8::String::new(scope, "BroadcastChannel").unwrap();
        global.define_own_property(scope, name_broadcast_channel.into(), ctor_broadcast_channel.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_capture_controller) = tmpl_capture_controller.get_function(scope) {
        let name_capture_controller = v8::String::new(scope, "CaptureController").unwrap();
        global.define_own_property(scope, name_capture_controller.into(), ctor_capture_controller.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_clipboard) = tmpl_clipboard.get_function(scope) {
        let name_clipboard = v8::String::new(scope, "Clipboard").unwrap();
        global.define_own_property(scope, name_clipboard.into(), ctor_clipboard.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_close_watcher) = tmpl_close_watcher.get_function(scope) {
        let name_close_watcher = v8::String::new(scope, "CloseWatcher").unwrap();
        global.define_own_property(scope, name_close_watcher.into(), ctor_close_watcher.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cookie_store) = tmpl_cookie_store.get_function(scope) {
        let name_cookie_store = v8::String::new(scope, "CookieStore").unwrap();
        global.define_own_property(scope, name_cookie_store.into(), ctor_cookie_store.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_create_monitor) = tmpl_create_monitor.get_function(scope) {
        let name_create_monitor = v8::String::new(scope, "CreateMonitor").unwrap();
        global.define_own_property(scope, name_create_monitor.into(), ctor_create_monitor.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_device_posture) = tmpl_device_posture.get_function(scope) {
        let name_device_posture = v8::String::new(scope, "DevicePosture").unwrap();
        global.define_own_property(scope, name_device_posture.into(), ctor_device_posture.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_document_picture_in_picture) = tmpl_document_picture_in_picture.get_function(scope) {
        let name_document_picture_in_picture = v8::String::new(scope, "DocumentPictureInPicture").unwrap();
        global.define_own_property(scope, name_document_picture_in_picture.into(), ctor_document_picture_in_picture.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_edit_context) = tmpl_edit_context.get_function(scope) {
        let name_edit_context = v8::String::new(scope, "EditContext").unwrap();
        global.define_own_property(scope, name_edit_context.into(), ctor_edit_context.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_event_source) = tmpl_event_source.get_function(scope) {
        let name_event_source = v8::String::new(scope, "EventSource").unwrap();
        global.define_own_property(scope, name_event_source.into(), ctor_event_source.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_file_reader) = tmpl_file_reader.get_function(scope) {
        let name_file_reader = v8::String::new(scope, "FileReader").unwrap();
        global.define_own_property(scope, name_file_reader.into(), ctor_file_reader.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_font_face_set) = tmpl_font_face_set.get_function(scope) {
        let name_font_face_set = v8::String::new(scope, "FontFaceSet").unwrap();
        global.define_own_property(scope, name_font_face_set.into(), ctor_font_face_set.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpudevice) = tmpl_gpudevice.get_function(scope) {
        let name_gpudevice = v8::String::new(scope, "GPUDevice").unwrap();
        global.define_own_property(scope, name_gpudevice.into(), ctor_gpudevice.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_hid) = tmpl_hid.get_function(scope) {
        let name_hid = v8::String::new(scope, "HID").unwrap();
        global.define_own_property(scope, name_hid.into(), ctor_hid.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_hiddevice) = tmpl_hiddevice.get_function(scope) {
        let name_hiddevice = v8::String::new(scope, "HIDDevice").unwrap();
        global.define_own_property(scope, name_hiddevice.into(), ctor_hiddevice.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idbdatabase) = tmpl_idbdatabase.get_function(scope) {
        let name_idbdatabase = v8::String::new(scope, "IDBDatabase").unwrap();
        global.define_own_property(scope, name_idbdatabase.into(), ctor_idbdatabase.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idbrequest) = tmpl_idbrequest.get_function(scope) {
        let name_idbrequest = v8::String::new(scope, "IDBRequest").unwrap();
        global.define_own_property(scope, name_idbrequest.into(), ctor_idbrequest.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idbtransaction) = tmpl_idbtransaction.get_function(scope) {
        let name_idbtransaction = v8::String::new(scope, "IDBTransaction").unwrap();
        global.define_own_property(scope, name_idbtransaction.into(), ctor_idbtransaction.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idle_detector) = tmpl_idle_detector.get_function(scope) {
        let name_idle_detector = v8::String::new(scope, "IdleDetector").unwrap();
        global.define_own_property(scope, name_idle_detector.into(), ctor_idle_detector.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_keyboard) = tmpl_keyboard.get_function(scope) {
        let name_keyboard = v8::String::new(scope, "Keyboard").unwrap();
        global.define_own_property(scope, name_keyboard.into(), ctor_keyboard.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_language_model) = tmpl_language_model.get_function(scope) {
        let name_language_model = v8::String::new(scope, "LanguageModel").unwrap();
        global.define_own_property(scope, name_language_model.into(), ctor_language_model.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_midiaccess) = tmpl_midiaccess.get_function(scope) {
        let name_midiaccess = v8::String::new(scope, "MIDIAccess").unwrap();
        global.define_own_property(scope, name_midiaccess.into(), ctor_midiaccess.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_midiport) = tmpl_midiport.get_function(scope) {
        let name_midiport = v8::String::new(scope, "MIDIPort").unwrap();
        global.define_own_property(scope, name_midiport.into(), ctor_midiport.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_devices) = tmpl_media_devices.get_function(scope) {
        let name_media_devices = v8::String::new(scope, "MediaDevices").unwrap();
        global.define_own_property(scope, name_media_devices.into(), ctor_media_devices.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_key_session) = tmpl_media_key_session.get_function(scope) {
        let name_media_key_session = v8::String::new(scope, "MediaKeySession").unwrap();
        global.define_own_property(scope, name_media_key_session.into(), ctor_media_key_session.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_query_list) = tmpl_media_query_list.get_function(scope) {
        let name_media_query_list = v8::String::new(scope, "MediaQueryList").unwrap();
        global.define_own_property(scope, name_media_query_list.into(), ctor_media_query_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_recorder) = tmpl_media_recorder.get_function(scope) {
        let name_media_recorder = v8::String::new(scope, "MediaRecorder").unwrap();
        global.define_own_property(scope, name_media_recorder.into(), ctor_media_recorder.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_source) = tmpl_media_source.get_function(scope) {
        let name_media_source = v8::String::new(scope, "MediaSource").unwrap();
        global.define_own_property(scope, name_media_source.into(), ctor_media_source.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_stream) = tmpl_media_stream.get_function(scope) {
        let name_media_stream = v8::String::new(scope, "MediaStream").unwrap();
        global.define_own_property(scope, name_media_stream.into(), ctor_media_stream.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_stream_track) = tmpl_media_stream_track.get_function(scope) {
        let name_media_stream_track = v8::String::new(scope, "MediaStreamTrack").unwrap();
        global.define_own_property(scope, name_media_stream_track.into(), ctor_media_stream_track.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_message_port) = tmpl_message_port.get_function(scope) {
        let name_message_port = v8::String::new(scope, "MessagePort").unwrap();
        global.define_own_property(scope, name_message_port.into(), ctor_message_port.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_model_context) = tmpl_model_context.get_function(scope) {
        let name_model_context = v8::String::new(scope, "ModelContext").unwrap();
        global.define_own_property(scope, name_model_context.into(), ctor_model_context.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_ndefreader) = tmpl_ndefreader.get_function(scope) {
        let name_ndefreader = v8::String::new(scope, "NDEFReader").unwrap();
        global.define_own_property(scope, name_ndefreader.into(), ctor_ndefreader.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_named_flow) = tmpl_named_flow.get_function(scope) {
        let name_named_flow = v8::String::new(scope, "NamedFlow").unwrap();
        global.define_own_property(scope, name_named_flow.into(), ctor_named_flow.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_navigation) = tmpl_navigation.get_function(scope) {
        let name_navigation = v8::String::new(scope, "Navigation").unwrap();
        global.define_own_property(scope, name_navigation.into(), ctor_navigation.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_navigation_history_entry) = tmpl_navigation_history_entry.get_function(scope) {
        let name_navigation_history_entry = v8::String::new(scope, "NavigationHistoryEntry").unwrap();
        global.define_own_property(scope, name_navigation_history_entry.into(), ctor_navigation_history_entry.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_navigator_managed_data) = tmpl_navigator_managed_data.get_function(scope) {
        let name_navigator_managed_data = v8::String::new(scope, "NavigatorManagedData").unwrap();
        global.define_own_property(scope, name_navigator_managed_data.into(), ctor_navigator_managed_data.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_network_information) = tmpl_network_information.get_function(scope) {
        let name_network_information = v8::String::new(scope, "NetworkInformation").unwrap();
        global.define_own_property(scope, name_network_information.into(), ctor_network_information.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_node) = tmpl_node.get_function(scope) {
        let name_node = v8::String::new(scope, "Node").unwrap();
        global.define_own_property(scope, name_node.into(), ctor_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_notification) = tmpl_notification.get_function(scope) {
        let name_notification = v8::String::new(scope, "Notification").unwrap();
        global.define_own_property(scope, name_notification.into(), ctor_notification.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_offscreen_canvas) = tmpl_offscreen_canvas.get_function(scope) {
        let name_offscreen_canvas = v8::String::new(scope, "OffscreenCanvas").unwrap();
        global.define_own_property(scope, name_offscreen_canvas.into(), ctor_offscreen_canvas.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_payment_request) = tmpl_payment_request.get_function(scope) {
        let name_payment_request = v8::String::new(scope, "PaymentRequest").unwrap();
        global.define_own_property(scope, name_payment_request.into(), ctor_payment_request.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_payment_response) = tmpl_payment_response.get_function(scope) {
        let name_payment_response = v8::String::new(scope, "PaymentResponse").unwrap();
        global.define_own_property(scope, name_payment_response.into(), ctor_payment_response.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_performance) = tmpl_performance.get_function(scope) {
        let name_performance = v8::String::new(scope, "Performance").unwrap();
        global.define_own_property(scope, name_performance.into(), ctor_performance.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_permission_status) = tmpl_permission_status.get_function(scope) {
        let name_permission_status = v8::String::new(scope, "PermissionStatus").unwrap();
        global.define_own_property(scope, name_permission_status.into(), ctor_permission_status.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_picture_in_picture_window) = tmpl_picture_in_picture_window.get_function(scope) {
        let name_picture_in_picture_window = v8::String::new(scope, "PictureInPictureWindow").unwrap();
        global.define_own_property(scope, name_picture_in_picture_window.into(), ctor_picture_in_picture_window.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_portal_host) = tmpl_portal_host.get_function(scope) {
        let name_portal_host = v8::String::new(scope, "PortalHost").unwrap();
        global.define_own_property(scope, name_portal_host.into(), ctor_portal_host.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_preference_object) = tmpl_preference_object.get_function(scope) {
        let name_preference_object = v8::String::new(scope, "PreferenceObject").unwrap();
        global.define_own_property(scope, name_preference_object.into(), ctor_preference_object.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_presentation_availability) = tmpl_presentation_availability.get_function(scope) {
        let name_presentation_availability = v8::String::new(scope, "PresentationAvailability").unwrap();
        global.define_own_property(scope, name_presentation_availability.into(), ctor_presentation_availability.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_presentation_connection) = tmpl_presentation_connection.get_function(scope) {
        let name_presentation_connection = v8::String::new(scope, "PresentationConnection").unwrap();
        global.define_own_property(scope, name_presentation_connection.into(), ctor_presentation_connection.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_presentation_connection_list) = tmpl_presentation_connection_list.get_function(scope) {
        let name_presentation_connection_list = v8::String::new(scope, "PresentationConnectionList").unwrap();
        global.define_own_property(scope, name_presentation_connection_list.into(), ctor_presentation_connection_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_presentation_request) = tmpl_presentation_request.get_function(scope) {
        let name_presentation_request = v8::String::new(scope, "PresentationRequest").unwrap();
        global.define_own_property(scope, name_presentation_request.into(), ctor_presentation_request.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_profiler) = tmpl_profiler.get_function(scope) {
        let name_profiler = v8::String::new(scope, "Profiler").unwrap();
        global.define_own_property(scope, name_profiler.into(), ctor_profiler.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcdtmfsender) = tmpl_rtcdtmfsender.get_function(scope) {
        let name_rtcdtmfsender = v8::String::new(scope, "RTCDTMFSender").unwrap();
        global.define_own_property(scope, name_rtcdtmfsender.into(), ctor_rtcdtmfsender.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcdata_channel) = tmpl_rtcdata_channel.get_function(scope) {
        let name_rtcdata_channel = v8::String::new(scope, "RTCDataChannel").unwrap();
        global.define_own_property(scope, name_rtcdata_channel.into(), ctor_rtcdata_channel.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcdtls_transport) = tmpl_rtcdtls_transport.get_function(scope) {
        let name_rtcdtls_transport = v8::String::new(scope, "RTCDtlsTransport").unwrap();
        global.define_own_property(scope, name_rtcdtls_transport.into(), ctor_rtcdtls_transport.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcice_transport) = tmpl_rtcice_transport.get_function(scope) {
        let name_rtcice_transport = v8::String::new(scope, "RTCIceTransport").unwrap();
        global.define_own_property(scope, name_rtcice_transport.into(), ctor_rtcice_transport.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcpeer_connection) = tmpl_rtcpeer_connection.get_function(scope) {
        let name_rtcpeer_connection = v8::String::new(scope, "RTCPeerConnection").unwrap();
        global.define_own_property(scope, name_rtcpeer_connection.into(), ctor_rtcpeer_connection.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcrtp_sframe_decrypter) = tmpl_rtcrtp_sframe_decrypter.get_function(scope) {
        let name_rtcrtp_sframe_decrypter = v8::String::new(scope, "RTCRtpSFrameDecrypter").unwrap();
        global.define_own_property(scope, name_rtcrtp_sframe_decrypter.into(), ctor_rtcrtp_sframe_decrypter.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcrtp_script_transformer) = tmpl_rtcrtp_script_transformer.get_function(scope) {
        let name_rtcrtp_script_transformer = v8::String::new(scope, "RTCRtpScriptTransformer").unwrap();
        global.define_own_property(scope, name_rtcrtp_script_transformer.into(), ctor_rtcrtp_script_transformer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcsctp_transport) = tmpl_rtcsctp_transport.get_function(scope) {
        let name_rtcsctp_transport = v8::String::new(scope, "RTCSctpTransport").unwrap();
        global.define_own_property(scope, name_rtcsctp_transport.into(), ctor_rtcsctp_transport.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_remote_playback) = tmpl_remote_playback.get_function(scope) {
        let name_remote_playback = v8::String::new(scope, "RemotePlayback").unwrap();
        global.define_own_property(scope, name_remote_playback.into(), ctor_remote_playback.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_sframe_decrypter_stream) = tmpl_sframe_decrypter_stream.get_function(scope) {
        let name_sframe_decrypter_stream = v8::String::new(scope, "SFrameDecrypterStream").unwrap();
        global.define_own_property(scope, name_sframe_decrypter_stream.into(), ctor_sframe_decrypter_stream.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_screen_details) = tmpl_screen_details.get_function(scope) {
        let name_screen_details = v8::String::new(scope, "ScreenDetails").unwrap();
        global.define_own_property(scope, name_screen_details.into(), ctor_screen_details.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_screen_orientation) = tmpl_screen_orientation.get_function(scope) {
        let name_screen_orientation = v8::String::new(scope, "ScreenOrientation").unwrap();
        global.define_own_property(scope, name_screen_orientation.into(), ctor_screen_orientation.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_sensor) = tmpl_sensor.get_function(scope) {
        let name_sensor = v8::String::new(scope, "Sensor").unwrap();
        global.define_own_property(scope, name_sensor.into(), ctor_sensor.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_serial) = tmpl_serial.get_function(scope) {
        let name_serial = v8::String::new(scope, "Serial").unwrap();
        global.define_own_property(scope, name_serial.into(), ctor_serial.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_serial_port) = tmpl_serial_port.get_function(scope) {
        let name_serial_port = v8::String::new(scope, "SerialPort").unwrap();
        global.define_own_property(scope, name_serial_port.into(), ctor_serial_port.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_service_worker) = tmpl_service_worker.get_function(scope) {
        let name_service_worker = v8::String::new(scope, "ServiceWorker").unwrap();
        global.define_own_property(scope, name_service_worker.into(), ctor_service_worker.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_service_worker_container) = tmpl_service_worker_container.get_function(scope) {
        let name_service_worker_container = v8::String::new(scope, "ServiceWorkerContainer").unwrap();
        global.define_own_property(scope, name_service_worker_container.into(), ctor_service_worker_container.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_service_worker_registration) = tmpl_service_worker_registration.get_function(scope) {
        let name_service_worker_registration = v8::String::new(scope, "ServiceWorkerRegistration").unwrap();
        global.define_own_property(scope, name_service_worker_registration.into(), ctor_service_worker_registration.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_shared_worker) = tmpl_shared_worker.get_function(scope) {
        let name_shared_worker = v8::String::new(scope, "SharedWorker").unwrap();
        global.define_own_property(scope, name_shared_worker.into(), ctor_shared_worker.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_source_buffer) = tmpl_source_buffer.get_function(scope) {
        let name_source_buffer = v8::String::new(scope, "SourceBuffer").unwrap();
        global.define_own_property(scope, name_source_buffer.into(), ctor_source_buffer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_source_buffer_list) = tmpl_source_buffer_list.get_function(scope) {
        let name_source_buffer_list = v8::String::new(scope, "SourceBufferList").unwrap();
        global.define_own_property(scope, name_source_buffer_list.into(), ctor_source_buffer_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_speech_recognition) = tmpl_speech_recognition.get_function(scope) {
        let name_speech_recognition = v8::String::new(scope, "SpeechRecognition").unwrap();
        global.define_own_property(scope, name_speech_recognition.into(), ctor_speech_recognition.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_speech_synthesis) = tmpl_speech_synthesis.get_function(scope) {
        let name_speech_synthesis = v8::String::new(scope, "SpeechSynthesis").unwrap();
        global.define_own_property(scope, name_speech_synthesis.into(), ctor_speech_synthesis.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_speech_synthesis_utterance) = tmpl_speech_synthesis_utterance.get_function(scope) {
        let name_speech_synthesis_utterance = v8::String::new(scope, "SpeechSynthesisUtterance").unwrap();
        global.define_own_property(scope, name_speech_synthesis_utterance.into(), ctor_speech_synthesis_utterance.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_text_track) = tmpl_text_track.get_function(scope) {
        let name_text_track = v8::String::new(scope, "TextTrack").unwrap();
        global.define_own_property(scope, name_text_track.into(), ctor_text_track.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_text_track_cue) = tmpl_text_track_cue.get_function(scope) {
        let name_text_track_cue = v8::String::new(scope, "TextTrackCue").unwrap();
        global.define_own_property(scope, name_text_track_cue.into(), ctor_text_track_cue.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_text_track_list) = tmpl_text_track_list.get_function(scope) {
        let name_text_track_list = v8::String::new(scope, "TextTrackList").unwrap();
        global.define_own_property(scope, name_text_track_list.into(), ctor_text_track_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usb) = tmpl_usb.get_function(scope) {
        let name_usb = v8::String::new(scope, "USB").unwrap();
        global.define_own_property(scope, name_usb.into(), ctor_usb.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_video_decoder) = tmpl_video_decoder.get_function(scope) {
        let name_video_decoder = v8::String::new(scope, "VideoDecoder").unwrap();
        global.define_own_property(scope, name_video_decoder.into(), ctor_video_decoder.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_video_encoder) = tmpl_video_encoder.get_function(scope) {
        let name_video_encoder = v8::String::new(scope, "VideoEncoder").unwrap();
        global.define_own_property(scope, name_video_encoder.into(), ctor_video_encoder.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_video_track_list) = tmpl_video_track_list.get_function(scope) {
        let name_video_track_list = v8::String::new(scope, "VideoTrackList").unwrap();
        global.define_own_property(scope, name_video_track_list.into(), ctor_video_track_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_virtual_keyboard) = tmpl_virtual_keyboard.get_function(scope) {
        let name_virtual_keyboard = v8::String::new(scope, "VirtualKeyboard").unwrap();
        global.define_own_property(scope, name_virtual_keyboard.into(), ctor_virtual_keyboard.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_visual_viewport) = tmpl_visual_viewport.get_function(scope) {
        let name_visual_viewport = v8::String::new(scope, "VisualViewport").unwrap();
        global.define_own_property(scope, name_visual_viewport.into(), ctor_visual_viewport.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_wake_lock_sentinel) = tmpl_wake_lock_sentinel.get_function(scope) {
        let name_wake_lock_sentinel = v8::String::new(scope, "WakeLockSentinel").unwrap();
        global.define_own_property(scope, name_wake_lock_sentinel.into(), ctor_wake_lock_sentinel.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_socket) = tmpl_web_socket.get_function(scope) {
        let name_web_socket = v8::String::new(scope, "WebSocket").unwrap();
        global.define_own_property(scope, name_web_socket.into(), ctor_web_socket.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_window) = tmpl_window.get_function(scope) {
        let name_window = v8::String::new(scope, "Window").unwrap();
        global.define_own_property(scope, name_window.into(), ctor_window.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_window_controls_overlay) = tmpl_window_controls_overlay.get_function(scope) {
        let name_window_controls_overlay = v8::String::new(scope, "WindowControlsOverlay").unwrap();
        global.define_own_property(scope, name_window_controls_overlay.into(), ctor_window_controls_overlay.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_worker) = tmpl_worker.get_function(scope) {
        let name_worker = v8::String::new(scope, "Worker").unwrap();
        global.define_own_property(scope, name_worker.into(), ctor_worker.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_worker_global_scope) = tmpl_worker_global_scope.get_function(scope) {
        let name_worker_global_scope = v8::String::new(scope, "WorkerGlobalScope").unwrap();
        global.define_own_property(scope, name_worker_global_scope.into(), ctor_worker_global_scope.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xmlhttp_request_event_target) = tmpl_xmlhttp_request_event_target.get_function(scope) {
        let name_xmlhttp_request_event_target = v8::String::new(scope, "XMLHttpRequestEventTarget").unwrap();
        global.define_own_property(scope, name_xmlhttp_request_event_target.into(), ctor_xmlhttp_request_event_target.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrlayer) = tmpl_xrlayer.get_function(scope) {
        let name_xrlayer = v8::String::new(scope, "XRLayer").unwrap();
        global.define_own_property(scope, name_xrlayer.into(), ctor_xrlayer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrlight_probe) = tmpl_xrlight_probe.get_function(scope) {
        let name_xrlight_probe = v8::String::new(scope, "XRLightProbe").unwrap();
        global.define_own_property(scope, name_xrlight_probe.into(), ctor_xrlight_probe.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrsession) = tmpl_xrsession.get_function(scope) {
        let name_xrsession = v8::String::new(scope, "XRSession").unwrap();
        global.define_own_property(scope, name_xrsession.into(), ctor_xrsession.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrspace) = tmpl_xrspace.get_function(scope) {
        let name_xrspace = v8::String::new(scope, "XRSpace").unwrap();
        global.define_own_property(scope, name_xrspace.into(), ctor_xrspace.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrsystem) = tmpl_xrsystem.get_function(scope) {
        let name_xrsystem = v8::String::new(scope, "XRSystem").unwrap();
        global.define_own_property(scope, name_xrsystem.into(), ctor_xrsystem.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_file_system_directory_entry) = tmpl_file_system_directory_entry.get_function(scope) {
        let name_file_system_directory_entry = v8::String::new(scope, "FileSystemDirectoryEntry").unwrap();
        global.define_own_property(scope, name_file_system_directory_entry.into(), ctor_file_system_directory_entry.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_file_system_file_entry) = tmpl_file_system_file_entry.get_function(scope) {
        let name_file_system_file_entry = v8::String::new(scope, "FileSystemFileEntry").unwrap();
        global.define_own_property(scope, name_file_system_file_entry.into(), ctor_file_system_file_entry.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_file_system_directory_handle) = tmpl_file_system_directory_handle.get_function(scope) {
        let name_file_system_directory_handle = v8::String::new(scope, "FileSystemDirectoryHandle").unwrap();
        global.define_own_property(scope, name_file_system_directory_handle.into(), ctor_file_system_directory_handle.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_file_system_file_handle) = tmpl_file_system_file_handle.get_function(scope) {
        let name_file_system_file_handle = v8::String::new(scope, "FileSystemFileHandle").unwrap();
        global.define_own_property(scope, name_file_system_file_handle.into(), ctor_file_system_file_handle.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpuinternal_error) = tmpl_gpuinternal_error.get_function(scope) {
        let name_gpuinternal_error = v8::String::new(scope, "GPUInternalError").unwrap();
        global.define_own_property(scope, name_gpuinternal_error.into(), ctor_gpuinternal_error.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpuout_of_memory_error) = tmpl_gpuout_of_memory_error.get_function(scope) {
        let name_gpuout_of_memory_error = v8::String::new(scope, "GPUOutOfMemoryError").unwrap();
        global.define_own_property(scope, name_gpuout_of_memory_error.into(), ctor_gpuout_of_memory_error.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpuvalidation_error) = tmpl_gpuvalidation_error.get_function(scope) {
        let name_gpuvalidation_error = v8::String::new(scope, "GPUValidationError").unwrap();
        global.define_own_property(scope, name_gpuvalidation_error.into(), ctor_gpuvalidation_error.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_sequence_effect) = tmpl_sequence_effect.get_function(scope) {
        let name_sequence_effect = v8::String::new(scope, "SequenceEffect").unwrap();
        global.define_own_property(scope, name_sequence_effect.into(), ctor_sequence_effect.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlform_controls_collection) = tmpl_htmlform_controls_collection.get_function(scope) {
        let name_htmlform_controls_collection = v8::String::new(scope, "HTMLFormControlsCollection").unwrap();
        global.define_own_property(scope, name_htmlform_controls_collection.into(), ctor_htmlform_controls_collection.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmloptions_collection) = tmpl_htmloptions_collection.get_function(scope) {
        let name_htmloptions_collection = v8::String::new(scope, "HTMLOptionsCollection").unwrap();
        global.define_own_property(scope, name_htmloptions_collection.into(), ctor_htmloptions_collection.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idbcursor_with_value) = tmpl_idbcursor_with_value.get_function(scope) {
        let name_idbcursor_with_value = v8::String::new(scope, "IDBCursorWithValue").unwrap();
        global.define_own_property(scope, name_idbcursor_with_value.into(), ctor_idbcursor_with_value.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_input_device_info) = tmpl_input_device_info.get_function(scope) {
        let name_input_device_info = v8::String::new(scope, "InputDeviceInfo").unwrap();
        global.define_own_property(scope, name_input_device_info.into(), ctor_input_device_info.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_radio_node_list) = tmpl_radio_node_list.get_function(scope) {
        let name_radio_node_list = v8::String::new(scope, "RadioNodeList").unwrap();
        global.define_own_property(scope, name_radio_node_list.into(), ctor_radio_node_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_largest_contentful_paint) = tmpl_largest_contentful_paint.get_function(scope) {
        let name_largest_contentful_paint = v8::String::new(scope, "LargestContentfulPaint").unwrap();
        global.define_own_property(scope, name_largest_contentful_paint.into(), ctor_largest_contentful_paint.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_layout_shift) = tmpl_layout_shift.get_function(scope) {
        let name_layout_shift = v8::String::new(scope, "LayoutShift").unwrap();
        global.define_own_property(scope, name_layout_shift.into(), ctor_layout_shift.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_performance_container_timing) = tmpl_performance_container_timing.get_function(scope) {
        let name_performance_container_timing = v8::String::new(scope, "PerformanceContainerTiming").unwrap();
        global.define_own_property(scope, name_performance_container_timing.into(), ctor_performance_container_timing.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_performance_element_timing) = tmpl_performance_element_timing.get_function(scope) {
        let name_performance_element_timing = v8::String::new(scope, "PerformanceElementTiming").unwrap();
        global.define_own_property(scope, name_performance_element_timing.into(), ctor_performance_element_timing.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_performance_event_timing) = tmpl_performance_event_timing.get_function(scope) {
        let name_performance_event_timing = v8::String::new(scope, "PerformanceEventTiming").unwrap();
        global.define_own_property(scope, name_performance_event_timing.into(), ctor_performance_event_timing.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_performance_long_animation_frame_timing) = tmpl_performance_long_animation_frame_timing.get_function(scope) {
        let name_performance_long_animation_frame_timing = v8::String::new(scope, "PerformanceLongAnimationFrameTiming").unwrap();
        global.define_own_property(scope, name_performance_long_animation_frame_timing.into(), ctor_performance_long_animation_frame_timing.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_performance_long_task_timing) = tmpl_performance_long_task_timing.get_function(scope) {
        let name_performance_long_task_timing = v8::String::new(scope, "PerformanceLongTaskTiming").unwrap();
        global.define_own_property(scope, name_performance_long_task_timing.into(), ctor_performance_long_task_timing.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_performance_mark) = tmpl_performance_mark.get_function(scope) {
        let name_performance_mark = v8::String::new(scope, "PerformanceMark").unwrap();
        global.define_own_property(scope, name_performance_mark.into(), ctor_performance_mark.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_performance_measure) = tmpl_performance_measure.get_function(scope) {
        let name_performance_measure = v8::String::new(scope, "PerformanceMeasure").unwrap();
        global.define_own_property(scope, name_performance_measure.into(), ctor_performance_measure.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_performance_paint_timing) = tmpl_performance_paint_timing.get_function(scope) {
        let name_performance_paint_timing = v8::String::new(scope, "PerformancePaintTiming").unwrap();
        global.define_own_property(scope, name_performance_paint_timing.into(), ctor_performance_paint_timing.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_performance_resource_timing) = tmpl_performance_resource_timing.get_function(scope) {
        let name_performance_resource_timing = v8::String::new(scope, "PerformanceResourceTiming").unwrap();
        global.define_own_property(scope, name_performance_resource_timing.into(), ctor_performance_resource_timing.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_performance_script_timing) = tmpl_performance_script_timing.get_function(scope) {
        let name_performance_script_timing = v8::String::new(scope, "PerformanceScriptTiming").unwrap();
        global.define_own_property(scope, name_performance_script_timing.into(), ctor_performance_script_timing.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_task_attribution_timing) = tmpl_task_attribution_timing.get_function(scope) {
        let name_task_attribution_timing = v8::String::new(scope, "TaskAttributionTiming").unwrap();
        global.define_own_property(scope, name_task_attribution_timing.into(), ctor_task_attribution_timing.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_visibility_state_entry) = tmpl_visibility_state_entry.get_function(scope) {
        let name_visibility_state_entry = v8::String::new(scope, "VisibilityStateEntry").unwrap();
        global.define_own_property(scope, name_visibility_state_entry.into(), ctor_visibility_state_entry.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_transport_receive_stream) = tmpl_web_transport_receive_stream.get_function(scope) {
        let name_web_transport_receive_stream = v8::String::new(scope, "WebTransportReceiveStream").unwrap();
        global.define_own_property(scope, name_web_transport_receive_stream.into(), ctor_web_transport_receive_stream.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_screen_detailed) = tmpl_screen_detailed.get_function(scope) {
        let name_screen_detailed = v8::String::new(scope, "ScreenDetailed").unwrap();
        global.define_own_property(scope, name_screen_detailed.into(), ctor_screen_detailed.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_style_property_map) = tmpl_style_property_map.get_function(scope) {
        let name_style_property_map = v8::String::new(scope, "StylePropertyMap").unwrap();
        global.define_own_property(scope, name_style_property_map.into(), ctor_style_property_map.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssstyle_sheet) = tmpl_cssstyle_sheet.get_function(scope) {
        let name_cssstyle_sheet = v8::String::new(scope, "CSSStyleSheet").unwrap();
        global.define_own_property(scope, name_cssstyle_sheet.into(), ctor_cssstyle_sheet.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_glbuffer) = tmpl_web_glbuffer.get_function(scope) {
        let name_web_glbuffer = v8::String::new(scope, "WebGLBuffer").unwrap();
        global.define_own_property(scope, name_web_glbuffer.into(), ctor_web_glbuffer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_glframebuffer) = tmpl_web_glframebuffer.get_function(scope) {
        let name_web_glframebuffer = v8::String::new(scope, "WebGLFramebuffer").unwrap();
        global.define_own_property(scope, name_web_glframebuffer.into(), ctor_web_glframebuffer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_glprogram) = tmpl_web_glprogram.get_function(scope) {
        let name_web_glprogram = v8::String::new(scope, "WebGLProgram").unwrap();
        global.define_own_property(scope, name_web_glprogram.into(), ctor_web_glprogram.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_glquery) = tmpl_web_glquery.get_function(scope) {
        let name_web_glquery = v8::String::new(scope, "WebGLQuery").unwrap();
        global.define_own_property(scope, name_web_glquery.into(), ctor_web_glquery.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_glrenderbuffer) = tmpl_web_glrenderbuffer.get_function(scope) {
        let name_web_glrenderbuffer = v8::String::new(scope, "WebGLRenderbuffer").unwrap();
        global.define_own_property(scope, name_web_glrenderbuffer.into(), ctor_web_glrenderbuffer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_glsampler) = tmpl_web_glsampler.get_function(scope) {
        let name_web_glsampler = v8::String::new(scope, "WebGLSampler").unwrap();
        global.define_own_property(scope, name_web_glsampler.into(), ctor_web_glsampler.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_glshader) = tmpl_web_glshader.get_function(scope) {
        let name_web_glshader = v8::String::new(scope, "WebGLShader").unwrap();
        global.define_own_property(scope, name_web_glshader.into(), ctor_web_glshader.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_glsync) = tmpl_web_glsync.get_function(scope) {
        let name_web_glsync = v8::String::new(scope, "WebGLSync").unwrap();
        global.define_own_property(scope, name_web_glsync.into(), ctor_web_glsync.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_gltexture) = tmpl_web_gltexture.get_function(scope) {
        let name_web_gltexture = v8::String::new(scope, "WebGLTexture").unwrap();
        global.define_own_property(scope, name_web_gltexture.into(), ctor_web_gltexture.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    // WebGLTimerQueryEXT: NoInterfaceObject — skip global registration
    if let Some(ctor_web_gltransform_feedback) = tmpl_web_gltransform_feedback.get_function(scope) {
        let name_web_gltransform_feedback = v8::String::new(scope, "WebGLTransformFeedback").unwrap();
        global.define_own_property(scope, name_web_gltransform_feedback.into(), ctor_web_gltransform_feedback.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_glvertex_array_object) = tmpl_web_glvertex_array_object.get_function(scope) {
        let name_web_glvertex_array_object = v8::String::new(scope, "WebGLVertexArrayObject").unwrap();
        global.define_own_property(scope, name_web_glvertex_array_object.into(), ctor_web_glvertex_array_object.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    // WebGLVertexArrayObjectOES: NoInterfaceObject — skip global registration
    if let Some(ctor_audio_worklet) = tmpl_audio_worklet.get_function(scope) {
        let name_audio_worklet = v8::String::new(scope, "AudioWorklet").unwrap();
        global.define_own_property(scope, name_audio_worklet.into(), ctor_audio_worklet.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_animation_worklet_global_scope) = tmpl_animation_worklet_global_scope.get_function(scope) {
        let name_animation_worklet_global_scope = v8::String::new(scope, "AnimationWorkletGlobalScope").unwrap();
        global.define_own_property(scope, name_animation_worklet_global_scope.into(), ctor_animation_worklet_global_scope.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_audio_worklet_global_scope) = tmpl_audio_worklet_global_scope.get_function(scope) {
        let name_audio_worklet_global_scope = v8::String::new(scope, "AudioWorkletGlobalScope").unwrap();
        global.define_own_property(scope, name_audio_worklet_global_scope.into(), ctor_audio_worklet_global_scope.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_layout_worklet_global_scope) = tmpl_layout_worklet_global_scope.get_function(scope) {
        let name_layout_worklet_global_scope = v8::String::new(scope, "LayoutWorkletGlobalScope").unwrap();
        global.define_own_property(scope, name_layout_worklet_global_scope.into(), ctor_layout_worklet_global_scope.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_paint_worklet_global_scope) = tmpl_paint_worklet_global_scope.get_function(scope) {
        let name_paint_worklet_global_scope = v8::String::new(scope, "PaintWorkletGlobalScope").unwrap();
        global.define_own_property(scope, name_paint_worklet_global_scope.into(), ctor_paint_worklet_global_scope.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_file_system_writable_file_stream) = tmpl_file_system_writable_file_stream.get_function(scope) {
        let name_file_system_writable_file_stream = v8::String::new(scope, "FileSystemWritableFileStream").unwrap();
        global.define_own_property(scope, name_file_system_writable_file_stream.into(), ctor_file_system_writable_file_stream.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_transport_datagrams_writable) = tmpl_web_transport_datagrams_writable.get_function(scope) {
        let name_web_transport_datagrams_writable = v8::String::new(scope, "WebTransportDatagramsWritable").unwrap();
        global.define_own_property(scope, name_web_transport_datagrams_writable.into(), ctor_web_transport_datagrams_writable.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_transport_send_stream) = tmpl_web_transport_send_stream.get_function(scope) {
        let name_web_transport_send_stream = v8::String::new(scope, "WebTransportSendStream").unwrap();
        global.define_own_property(scope, name_web_transport_send_stream.into(), ctor_web_transport_send_stream.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_transport_writer) = tmpl_web_transport_writer.get_function(scope) {
        let name_web_transport_writer = v8::String::new(scope, "WebTransportWriter").unwrap();
        global.define_own_property(scope, name_web_transport_writer.into(), ctor_web_transport_writer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrcpudepth_information) = tmpl_xrcpudepth_information.get_function(scope) {
        let name_xrcpudepth_information = v8::String::new(scope, "XRCPUDepthInformation").unwrap();
        global.define_own_property(scope, name_xrcpudepth_information.into(), ctor_xrcpudepth_information.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrweb_gldepth_information) = tmpl_xrweb_gldepth_information.get_function(scope) {
        let name_xrweb_gldepth_information = v8::String::new(scope, "XRWebGLDepthInformation").unwrap();
        global.define_own_property(scope, name_xrweb_gldepth_information.into(), ctor_xrweb_gldepth_information.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrjoint_pose) = tmpl_xrjoint_pose.get_function(scope) {
        let name_xrjoint_pose = v8::String::new(scope, "XRJointPose").unwrap();
        global.define_own_property(scope, name_xrjoint_pose.into(), ctor_xrjoint_pose.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrviewer_pose) = tmpl_xrviewer_pose.get_function(scope) {
        let name_xrviewer_pose = v8::String::new(scope, "XRViewerPose").unwrap();
        global.define_own_property(scope, name_xrviewer_pose.into(), ctor_xrviewer_pose.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrweb_glsub_image) = tmpl_xrweb_glsub_image.get_function(scope) {
        let name_xrweb_glsub_image = v8::String::new(scope, "XRWebGLSubImage").unwrap();
        global.define_own_property(scope, name_xrweb_glsub_image.into(), ctor_xrweb_glsub_image.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_view_timeline) = tmpl_view_timeline.get_function(scope) {
        let name_view_timeline = v8::String::new(scope, "ViewTimeline").unwrap();
        global.define_own_property(scope, name_view_timeline.into(), ctor_view_timeline.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssapply_block_rule) = tmpl_cssapply_block_rule.get_function(scope) {
        let name_cssapply_block_rule = v8::String::new(scope, "CSSApplyBlockRule").unwrap();
        global.define_own_property(scope, name_cssapply_block_rule.into(), ctor_cssapply_block_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csscondition_rule) = tmpl_csscondition_rule.get_function(scope) {
        let name_csscondition_rule = v8::String::new(scope, "CSSConditionRule").unwrap();
        global.define_own_property(scope, name_csscondition_rule.into(), ctor_csscondition_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csscontents_block_rule) = tmpl_csscontents_block_rule.get_function(scope) {
        let name_csscontents_block_rule = v8::String::new(scope, "CSSContentsBlockRule").unwrap();
        global.define_own_property(scope, name_csscontents_block_rule.into(), ctor_csscontents_block_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssfunction_rule) = tmpl_cssfunction_rule.get_function(scope) {
        let name_cssfunction_rule = v8::String::new(scope, "CSSFunctionRule").unwrap();
        global.define_own_property(scope, name_cssfunction_rule.into(), ctor_cssfunction_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csslayer_block_rule) = tmpl_csslayer_block_rule.get_function(scope) {
        let name_csslayer_block_rule = v8::String::new(scope, "CSSLayerBlockRule").unwrap();
        global.define_own_property(scope, name_csslayer_block_rule.into(), ctor_csslayer_block_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssmixin_rule) = tmpl_cssmixin_rule.get_function(scope) {
        let name_cssmixin_rule = v8::String::new(scope, "CSSMixinRule").unwrap();
        global.define_own_property(scope, name_cssmixin_rule.into(), ctor_cssmixin_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csspage_rule) = tmpl_csspage_rule.get_function(scope) {
        let name_csspage_rule = v8::String::new(scope, "CSSPageRule").unwrap();
        global.define_own_property(scope, name_csspage_rule.into(), ctor_csspage_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssscope_rule) = tmpl_cssscope_rule.get_function(scope) {
        let name_cssscope_rule = v8::String::new(scope, "CSSScopeRule").unwrap();
        global.define_own_property(scope, name_cssscope_rule.into(), ctor_cssscope_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssstarting_style_rule) = tmpl_cssstarting_style_rule.get_function(scope) {
        let name_cssstarting_style_rule = v8::String::new(scope, "CSSStartingStyleRule").unwrap();
        global.define_own_property(scope, name_cssstarting_style_rule.into(), ctor_cssstarting_style_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssstyle_rule) = tmpl_cssstyle_rule.get_function(scope) {
        let name_cssstyle_rule = v8::String::new(scope, "CSSStyleRule").unwrap();
        global.define_own_property(scope, name_cssstyle_rule.into(), ctor_cssstyle_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csssupports_condition_rule) = tmpl_csssupports_condition_rule.get_function(scope) {
        let name_csssupports_condition_rule = v8::String::new(scope, "CSSSupportsConditionRule").unwrap();
        global.define_own_property(scope, name_csssupports_condition_rule.into(), ctor_csssupports_condition_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csscolor) = tmpl_csscolor.get_function(scope) {
        let name_csscolor = v8::String::new(scope, "CSSColor").unwrap();
        global.define_own_property(scope, name_csscolor.into(), ctor_csscolor.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csshsl) = tmpl_csshsl.get_function(scope) {
        let name_csshsl = v8::String::new(scope, "CSSHSL").unwrap();
        global.define_own_property(scope, name_csshsl.into(), ctor_csshsl.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csshwb) = tmpl_csshwb.get_function(scope) {
        let name_csshwb = v8::String::new(scope, "CSSHWB").unwrap();
        global.define_own_property(scope, name_csshwb.into(), ctor_csshwb.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csslch) = tmpl_csslch.get_function(scope) {
        let name_csslch = v8::String::new(scope, "CSSLCH").unwrap();
        global.define_own_property(scope, name_csslch.into(), ctor_csslch.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csslab) = tmpl_csslab.get_function(scope) {
        let name_csslab = v8::String::new(scope, "CSSLab").unwrap();
        global.define_own_property(scope, name_csslab.into(), ctor_csslab.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssoklch) = tmpl_cssoklch.get_function(scope) {
        let name_cssoklch = v8::String::new(scope, "CSSOKLCH").unwrap();
        global.define_own_property(scope, name_cssoklch.into(), ctor_cssoklch.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssoklab) = tmpl_cssoklab.get_function(scope) {
        let name_cssoklab = v8::String::new(scope, "CSSOKLab").unwrap();
        global.define_own_property(scope, name_cssoklab.into(), ctor_cssoklab.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssrgb) = tmpl_cssrgb.get_function(scope) {
        let name_cssrgb = v8::String::new(scope, "CSSRGB").unwrap();
        global.define_own_property(scope, name_cssrgb.into(), ctor_cssrgb.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssmath_value) = tmpl_cssmath_value.get_function(scope) {
        let name_cssmath_value = v8::String::new(scope, "CSSMathValue").unwrap();
        global.define_own_property(scope, name_cssmath_value.into(), ctor_cssmath_value.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssunit_value) = tmpl_cssunit_value.get_function(scope) {
        let name_cssunit_value = v8::String::new(scope, "CSSUnitValue").unwrap();
        global.define_own_property(scope, name_cssunit_value.into(), ctor_cssunit_value.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_background_fetch_event) = tmpl_background_fetch_event.get_function(scope) {
        let name_background_fetch_event = v8::String::new(scope, "BackgroundFetchEvent").unwrap();
        global.define_own_property(scope, name_background_fetch_event.into(), ctor_background_fetch_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_can_make_payment_event) = tmpl_can_make_payment_event.get_function(scope) {
        let name_can_make_payment_event = v8::String::new(scope, "CanMakePaymentEvent").unwrap();
        global.define_own_property(scope, name_can_make_payment_event.into(), ctor_can_make_payment_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_content_index_event) = tmpl_content_index_event.get_function(scope) {
        let name_content_index_event = v8::String::new(scope, "ContentIndexEvent").unwrap();
        global.define_own_property(scope, name_content_index_event.into(), ctor_content_index_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_extendable_cookie_change_event) = tmpl_extendable_cookie_change_event.get_function(scope) {
        let name_extendable_cookie_change_event = v8::String::new(scope, "ExtendableCookieChangeEvent").unwrap();
        global.define_own_property(scope, name_extendable_cookie_change_event.into(), ctor_extendable_cookie_change_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_extendable_message_event) = tmpl_extendable_message_event.get_function(scope) {
        let name_extendable_message_event = v8::String::new(scope, "ExtendableMessageEvent").unwrap();
        global.define_own_property(scope, name_extendable_message_event.into(), ctor_extendable_message_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_fetch_event) = tmpl_fetch_event.get_function(scope) {
        let name_fetch_event = v8::String::new(scope, "FetchEvent").unwrap();
        global.define_own_property(scope, name_fetch_event.into(), ctor_fetch_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_install_event) = tmpl_install_event.get_function(scope) {
        let name_install_event = v8::String::new(scope, "InstallEvent").unwrap();
        global.define_own_property(scope, name_install_event.into(), ctor_install_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_notification_event) = tmpl_notification_event.get_function(scope) {
        let name_notification_event = v8::String::new(scope, "NotificationEvent").unwrap();
        global.define_own_property(scope, name_notification_event.into(), ctor_notification_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_payment_request_event) = tmpl_payment_request_event.get_function(scope) {
        let name_payment_request_event = v8::String::new(scope, "PaymentRequestEvent").unwrap();
        global.define_own_property(scope, name_payment_request_event.into(), ctor_payment_request_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_periodic_sync_event) = tmpl_periodic_sync_event.get_function(scope) {
        let name_periodic_sync_event = v8::String::new(scope, "PeriodicSyncEvent").unwrap();
        global.define_own_property(scope, name_periodic_sync_event.into(), ctor_periodic_sync_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_push_event) = tmpl_push_event.get_function(scope) {
        let name_push_event = v8::String::new(scope, "PushEvent").unwrap();
        global.define_own_property(scope, name_push_event.into(), ctor_push_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_push_subscription_change_event) = tmpl_push_subscription_change_event.get_function(scope) {
        let name_push_subscription_change_event = v8::String::new(scope, "PushSubscriptionChangeEvent").unwrap();
        global.define_own_property(scope, name_push_subscription_change_event.into(), ctor_push_subscription_change_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_sync_event) = tmpl_sync_event.get_function(scope) {
        let name_sync_event = v8::String::new(scope, "SyncEvent").unwrap();
        global.define_own_property(scope, name_sync_event.into(), ctor_sync_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_payment_method_change_event) = tmpl_payment_method_change_event.get_function(scope) {
        let name_payment_method_change_event = v8::String::new(scope, "PaymentMethodChangeEvent").unwrap();
        global.define_own_property(scope, name_payment_method_change_event.into(), ctor_payment_method_change_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_speech_synthesis_error_event) = tmpl_speech_synthesis_error_event.get_function(scope) {
        let name_speech_synthesis_error_event = v8::String::new(scope, "SpeechSynthesisErrorEvent").unwrap();
        global.define_own_property(scope, name_speech_synthesis_error_event.into(), ctor_speech_synthesis_error_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_composition_event) = tmpl_composition_event.get_function(scope) {
        let name_composition_event = v8::String::new(scope, "CompositionEvent").unwrap();
        global.define_own_property(scope, name_composition_event.into(), ctor_composition_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_focus_event) = tmpl_focus_event.get_function(scope) {
        let name_focus_event = v8::String::new(scope, "FocusEvent").unwrap();
        global.define_own_property(scope, name_focus_event.into(), ctor_focus_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_input_event) = tmpl_input_event.get_function(scope) {
        let name_input_event = v8::String::new(scope, "InputEvent").unwrap();
        global.define_own_property(scope, name_input_event.into(), ctor_input_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_keyboard_event) = tmpl_keyboard_event.get_function(scope) {
        let name_keyboard_event = v8::String::new(scope, "KeyboardEvent").unwrap();
        global.define_own_property(scope, name_keyboard_event.into(), ctor_keyboard_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_mouse_event) = tmpl_mouse_event.get_function(scope) {
        let name_mouse_event = v8::String::new(scope, "MouseEvent").unwrap();
        global.define_own_property(scope, name_mouse_event.into(), ctor_mouse_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_navigation_event) = tmpl_navigation_event.get_function(scope) {
        let name_navigation_event = v8::String::new(scope, "NavigationEvent").unwrap();
        global.define_own_property(scope, name_navigation_event.into(), ctor_navigation_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_text_event) = tmpl_text_event.get_function(scope) {
        let name_text_event = v8::String::new(scope, "TextEvent").unwrap();
        global.define_own_property(scope, name_text_event.into(), ctor_text_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_touch_event) = tmpl_touch_event.get_function(scope) {
        let name_touch_event = v8::String::new(scope, "TouchEvent").unwrap();
        global.define_own_property(scope, name_touch_event.into(), ctor_touch_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_task_signal) = tmpl_task_signal.get_function(scope) {
        let name_task_signal = v8::String::new(scope, "TaskSignal").unwrap();
        global.define_own_property(scope, name_task_signal.into(), ctor_task_signal.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssanimation) = tmpl_cssanimation.get_function(scope) {
        let name_cssanimation = v8::String::new(scope, "CSSAnimation").unwrap();
        global.define_own_property(scope, name_cssanimation.into(), ctor_cssanimation.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csstransition) = tmpl_csstransition.get_function(scope) {
        let name_csstransition = v8::String::new(scope, "CSSTransition").unwrap();
        global.define_own_property(scope, name_csstransition.into(), ctor_csstransition.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_shadow_animation) = tmpl_shadow_animation.get_function(scope) {
        let name_shadow_animation = v8::String::new(scope, "ShadowAnimation").unwrap();
        global.define_own_property(scope, name_shadow_animation.into(), ctor_shadow_animation.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_worklet_animation) = tmpl_worklet_animation.get_function(scope) {
        let name_worklet_animation = v8::String::new(scope, "WorkletAnimation").unwrap();
        global.define_own_property(scope, name_worklet_animation.into(), ctor_worklet_animation.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_analyser_node) = tmpl_analyser_node.get_function(scope) {
        let name_analyser_node = v8::String::new(scope, "AnalyserNode").unwrap();
        global.define_own_property(scope, name_analyser_node.into(), ctor_analyser_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_audio_destination_node) = tmpl_audio_destination_node.get_function(scope) {
        let name_audio_destination_node = v8::String::new(scope, "AudioDestinationNode").unwrap();
        global.define_own_property(scope, name_audio_destination_node.into(), ctor_audio_destination_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_audio_scheduled_source_node) = tmpl_audio_scheduled_source_node.get_function(scope) {
        let name_audio_scheduled_source_node = v8::String::new(scope, "AudioScheduledSourceNode").unwrap();
        global.define_own_property(scope, name_audio_scheduled_source_node.into(), ctor_audio_scheduled_source_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_audio_worklet_node) = tmpl_audio_worklet_node.get_function(scope) {
        let name_audio_worklet_node = v8::String::new(scope, "AudioWorkletNode").unwrap();
        global.define_own_property(scope, name_audio_worklet_node.into(), ctor_audio_worklet_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_biquad_filter_node) = tmpl_biquad_filter_node.get_function(scope) {
        let name_biquad_filter_node = v8::String::new(scope, "BiquadFilterNode").unwrap();
        global.define_own_property(scope, name_biquad_filter_node.into(), ctor_biquad_filter_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_channel_merger_node) = tmpl_channel_merger_node.get_function(scope) {
        let name_channel_merger_node = v8::String::new(scope, "ChannelMergerNode").unwrap();
        global.define_own_property(scope, name_channel_merger_node.into(), ctor_channel_merger_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_channel_splitter_node) = tmpl_channel_splitter_node.get_function(scope) {
        let name_channel_splitter_node = v8::String::new(scope, "ChannelSplitterNode").unwrap();
        global.define_own_property(scope, name_channel_splitter_node.into(), ctor_channel_splitter_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_convolver_node) = tmpl_convolver_node.get_function(scope) {
        let name_convolver_node = v8::String::new(scope, "ConvolverNode").unwrap();
        global.define_own_property(scope, name_convolver_node.into(), ctor_convolver_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_delay_node) = tmpl_delay_node.get_function(scope) {
        let name_delay_node = v8::String::new(scope, "DelayNode").unwrap();
        global.define_own_property(scope, name_delay_node.into(), ctor_delay_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_dynamics_compressor_node) = tmpl_dynamics_compressor_node.get_function(scope) {
        let name_dynamics_compressor_node = v8::String::new(scope, "DynamicsCompressorNode").unwrap();
        global.define_own_property(scope, name_dynamics_compressor_node.into(), ctor_dynamics_compressor_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gain_node) = tmpl_gain_node.get_function(scope) {
        let name_gain_node = v8::String::new(scope, "GainNode").unwrap();
        global.define_own_property(scope, name_gain_node.into(), ctor_gain_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_iirfilter_node) = tmpl_iirfilter_node.get_function(scope) {
        let name_iirfilter_node = v8::String::new(scope, "IIRFilterNode").unwrap();
        global.define_own_property(scope, name_iirfilter_node.into(), ctor_iirfilter_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_element_audio_source_node) = tmpl_media_element_audio_source_node.get_function(scope) {
        let name_media_element_audio_source_node = v8::String::new(scope, "MediaElementAudioSourceNode").unwrap();
        global.define_own_property(scope, name_media_element_audio_source_node.into(), ctor_media_element_audio_source_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_stream_audio_destination_node) = tmpl_media_stream_audio_destination_node.get_function(scope) {
        let name_media_stream_audio_destination_node = v8::String::new(scope, "MediaStreamAudioDestinationNode").unwrap();
        global.define_own_property(scope, name_media_stream_audio_destination_node.into(), ctor_media_stream_audio_destination_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_stream_audio_source_node) = tmpl_media_stream_audio_source_node.get_function(scope) {
        let name_media_stream_audio_source_node = v8::String::new(scope, "MediaStreamAudioSourceNode").unwrap();
        global.define_own_property(scope, name_media_stream_audio_source_node.into(), ctor_media_stream_audio_source_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_media_stream_track_audio_source_node) = tmpl_media_stream_track_audio_source_node.get_function(scope) {
        let name_media_stream_track_audio_source_node = v8::String::new(scope, "MediaStreamTrackAudioSourceNode").unwrap();
        global.define_own_property(scope, name_media_stream_track_audio_source_node.into(), ctor_media_stream_track_audio_source_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_panner_node) = tmpl_panner_node.get_function(scope) {
        let name_panner_node = v8::String::new(scope, "PannerNode").unwrap();
        global.define_own_property(scope, name_panner_node.into(), ctor_panner_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_script_processor_node) = tmpl_script_processor_node.get_function(scope) {
        let name_script_processor_node = v8::String::new(scope, "ScriptProcessorNode").unwrap();
        global.define_own_property(scope, name_script_processor_node.into(), ctor_script_processor_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_stereo_panner_node) = tmpl_stereo_panner_node.get_function(scope) {
        let name_stereo_panner_node = v8::String::new(scope, "StereoPannerNode").unwrap();
        global.define_own_property(scope, name_stereo_panner_node.into(), ctor_stereo_panner_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_wave_shaper_node) = tmpl_wave_shaper_node.get_function(scope) {
        let name_wave_shaper_node = v8::String::new(scope, "WaveShaperNode").unwrap();
        global.define_own_property(scope, name_wave_shaper_node.into(), ctor_wave_shaper_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_audio_context) = tmpl_audio_context.get_function(scope) {
        let name_audio_context = v8::String::new(scope, "AudioContext").unwrap();
        global.define_own_property(scope, name_audio_context.into(), ctor_audio_context.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_offline_audio_context) = tmpl_offline_audio_context.get_function(scope) {
        let name_offline_audio_context = v8::String::new(scope, "OfflineAudioContext").unwrap();
        global.define_own_property(scope, name_offline_audio_context.into(), ctor_offline_audio_context.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idbopen_dbrequest) = tmpl_idbopen_dbrequest.get_function(scope) {
        let name_idbopen_dbrequest = v8::String::new(scope, "IDBOpenDBRequest").unwrap();
        global.define_own_property(scope, name_idbopen_dbrequest.into(), ctor_idbopen_dbrequest.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_midiinput) = tmpl_midiinput.get_function(scope) {
        let name_midiinput = v8::String::new(scope, "MIDIInput").unwrap();
        global.define_own_property(scope, name_midiinput.into(), ctor_midiinput.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_midioutput) = tmpl_midioutput.get_function(scope) {
        let name_midioutput = v8::String::new(scope, "MIDIOutput").unwrap();
        global.define_own_property(scope, name_midioutput.into(), ctor_midioutput.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_managed_media_source) = tmpl_managed_media_source.get_function(scope) {
        let name_managed_media_source = v8::String::new(scope, "ManagedMediaSource").unwrap();
        global.define_own_property(scope, name_managed_media_source.into(), ctor_managed_media_source.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_browser_capture_media_stream_track) = tmpl_browser_capture_media_stream_track.get_function(scope) {
        let name_browser_capture_media_stream_track = v8::String::new(scope, "BrowserCaptureMediaStreamTrack").unwrap();
        global.define_own_property(scope, name_browser_capture_media_stream_track.into(), ctor_browser_capture_media_stream_track.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_canvas_capture_media_stream_track) = tmpl_canvas_capture_media_stream_track.get_function(scope) {
        let name_canvas_capture_media_stream_track = v8::String::new(scope, "CanvasCaptureMediaStreamTrack").unwrap();
        global.define_own_property(scope, name_canvas_capture_media_stream_track.into(), ctor_canvas_capture_media_stream_track.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_attr) = tmpl_attr.get_function(scope) {
        let name_attr = v8::String::new(scope, "Attr").unwrap();
        global.define_own_property(scope, name_attr.into(), ctor_attr.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_character_data) = tmpl_character_data.get_function(scope) {
        let name_character_data = v8::String::new(scope, "CharacterData").unwrap();
        global.define_own_property(scope, name_character_data.into(), ctor_character_data.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_document) = tmpl_document.get_function(scope) {
        let name_document = v8::String::new(scope, "Document").unwrap();
        global.define_own_property(scope, name_document.into(), ctor_document.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_document_fragment) = tmpl_document_fragment.get_function(scope) {
        let name_document_fragment = v8::String::new(scope, "DocumentFragment").unwrap();
        global.define_own_property(scope, name_document_fragment.into(), ctor_document_fragment.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_document_type) = tmpl_document_type.get_function(scope) {
        let name_document_type = v8::String::new(scope, "DocumentType").unwrap();
        global.define_own_property(scope, name_document_type.into(), ctor_document_type.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_element) = tmpl_element.get_function(scope) {
        let name_element = v8::String::new(scope, "Element").unwrap();
        global.define_own_property(scope, name_element.into(), ctor_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_lescan_permission_result) = tmpl_bluetooth_lescan_permission_result.get_function(scope) {
        let name_bluetooth_lescan_permission_result = v8::String::new(scope, "BluetoothLEScanPermissionResult").unwrap();
        global.define_own_property(scope, name_bluetooth_lescan_permission_result.into(), ctor_bluetooth_lescan_permission_result.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_permission_result) = tmpl_bluetooth_permission_result.get_function(scope) {
        let name_bluetooth_permission_result = v8::String::new(scope, "BluetoothPermissionResult").unwrap();
        global.define_own_property(scope, name_bluetooth_permission_result.into(), ctor_bluetooth_permission_result.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usbpermission_result) = tmpl_usbpermission_result.get_function(scope) {
        let name_usbpermission_result = v8::String::new(scope, "USBPermissionResult").unwrap();
        global.define_own_property(scope, name_usbpermission_result.into(), ctor_usbpermission_result.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrpermission_status) = tmpl_xrpermission_status.get_function(scope) {
        let name_xrpermission_status = v8::String::new(scope, "XRPermissionStatus").unwrap();
        global.define_own_property(scope, name_xrpermission_status.into(), ctor_xrpermission_status.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_accelerometer) = tmpl_accelerometer.get_function(scope) {
        let name_accelerometer = v8::String::new(scope, "Accelerometer").unwrap();
        global.define_own_property(scope, name_accelerometer.into(), ctor_accelerometer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_ambient_light_sensor) = tmpl_ambient_light_sensor.get_function(scope) {
        let name_ambient_light_sensor = v8::String::new(scope, "AmbientLightSensor").unwrap();
        global.define_own_property(scope, name_ambient_light_sensor.into(), ctor_ambient_light_sensor.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gyroscope) = tmpl_gyroscope.get_function(scope) {
        let name_gyroscope = v8::String::new(scope, "Gyroscope").unwrap();
        global.define_own_property(scope, name_gyroscope.into(), ctor_gyroscope.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_magnetometer) = tmpl_magnetometer.get_function(scope) {
        let name_magnetometer = v8::String::new(scope, "Magnetometer").unwrap();
        global.define_own_property(scope, name_magnetometer.into(), ctor_magnetometer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_orientation_sensor) = tmpl_orientation_sensor.get_function(scope) {
        let name_orientation_sensor = v8::String::new(scope, "OrientationSensor").unwrap();
        global.define_own_property(scope, name_orientation_sensor.into(), ctor_orientation_sensor.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_proximity_sensor) = tmpl_proximity_sensor.get_function(scope) {
        let name_proximity_sensor = v8::String::new(scope, "ProximitySensor").unwrap();
        global.define_own_property(scope, name_proximity_sensor.into(), ctor_proximity_sensor.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_uncalibrated_magnetometer) = tmpl_uncalibrated_magnetometer.get_function(scope) {
        let name_uncalibrated_magnetometer = v8::String::new(scope, "UncalibratedMagnetometer").unwrap();
        global.define_own_property(scope, name_uncalibrated_magnetometer.into(), ctor_uncalibrated_magnetometer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_managed_source_buffer) = tmpl_managed_source_buffer.get_function(scope) {
        let name_managed_source_buffer = v8::String::new(scope, "ManagedSourceBuffer").unwrap();
        global.define_own_property(scope, name_managed_source_buffer.into(), ctor_managed_source_buffer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_data_cue) = tmpl_data_cue.get_function(scope) {
        let name_data_cue = v8::String::new(scope, "DataCue").unwrap();
        global.define_own_property(scope, name_data_cue.into(), ctor_data_cue.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_vttcue) = tmpl_vttcue.get_function(scope) {
        let name_vttcue = v8::String::new(scope, "VTTCue").unwrap();
        global.define_own_property(scope, name_vttcue.into(), ctor_vttcue.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_dedicated_worker_global_scope) = tmpl_dedicated_worker_global_scope.get_function(scope) {
        let name_dedicated_worker_global_scope = v8::String::new(scope, "DedicatedWorkerGlobalScope").unwrap();
        global.define_own_property(scope, name_dedicated_worker_global_scope.into(), ctor_dedicated_worker_global_scope.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtcidentity_provider_global_scope) = tmpl_rtcidentity_provider_global_scope.get_function(scope) {
        let name_rtcidentity_provider_global_scope = v8::String::new(scope, "RTCIdentityProviderGlobalScope").unwrap();
        global.define_own_property(scope, name_rtcidentity_provider_global_scope.into(), ctor_rtcidentity_provider_global_scope.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_service_worker_global_scope) = tmpl_service_worker_global_scope.get_function(scope) {
        let name_service_worker_global_scope = v8::String::new(scope, "ServiceWorkerGlobalScope").unwrap();
        global.define_own_property(scope, name_service_worker_global_scope.into(), ctor_service_worker_global_scope.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_shared_worker_global_scope) = tmpl_shared_worker_global_scope.get_function(scope) {
        let name_shared_worker_global_scope = v8::String::new(scope, "SharedWorkerGlobalScope").unwrap();
        global.define_own_property(scope, name_shared_worker_global_scope.into(), ctor_shared_worker_global_scope.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xmlhttp_request) = tmpl_xmlhttp_request.get_function(scope) {
        let name_xmlhttp_request = v8::String::new(scope, "XMLHttpRequest").unwrap();
        global.define_own_property(scope, name_xmlhttp_request.into(), ctor_xmlhttp_request.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xmlhttp_request_upload) = tmpl_xmlhttp_request_upload.get_function(scope) {
        let name_xmlhttp_request_upload = v8::String::new(scope, "XMLHttpRequestUpload").unwrap();
        global.define_own_property(scope, name_xmlhttp_request_upload.into(), ctor_xmlhttp_request_upload.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrcomposition_layer) = tmpl_xrcomposition_layer.get_function(scope) {
        let name_xrcomposition_layer = v8::String::new(scope, "XRCompositionLayer").unwrap();
        global.define_own_property(scope, name_xrcomposition_layer.into(), ctor_xrcomposition_layer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrweb_gllayer) = tmpl_xrweb_gllayer.get_function(scope) {
        let name_xrweb_gllayer = v8::String::new(scope, "XRWebGLLayer").unwrap();
        global.define_own_property(scope, name_xrweb_gllayer.into(), ctor_xrweb_gllayer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrbody_space) = tmpl_xrbody_space.get_function(scope) {
        let name_xrbody_space = v8::String::new(scope, "XRBodySpace").unwrap();
        global.define_own_property(scope, name_xrbody_space.into(), ctor_xrbody_space.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrjoint_space) = tmpl_xrjoint_space.get_function(scope) {
        let name_xrjoint_space = v8::String::new(scope, "XRJointSpace").unwrap();
        global.define_own_property(scope, name_xrjoint_space.into(), ctor_xrjoint_space.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrreference_space) = tmpl_xrreference_space.get_function(scope) {
        let name_xrreference_space = v8::String::new(scope, "XRReferenceSpace").unwrap();
        global.define_own_property(scope, name_xrreference_space.into(), ctor_xrreference_space.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_performance_navigation_timing) = tmpl_performance_navigation_timing.get_function(scope) {
        let name_performance_navigation_timing = v8::String::new(scope, "PerformanceNavigationTiming").unwrap();
        global.define_own_property(scope, name_performance_navigation_timing.into(), ctor_performance_navigation_timing.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csscontainer_rule) = tmpl_csscontainer_rule.get_function(scope) {
        let name_csscontainer_rule = v8::String::new(scope, "CSSContainerRule").unwrap();
        global.define_own_property(scope, name_csscontainer_rule.into(), ctor_csscontainer_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssmedia_rule) = tmpl_cssmedia_rule.get_function(scope) {
        let name_cssmedia_rule = v8::String::new(scope, "CSSMediaRule").unwrap();
        global.define_own_property(scope, name_cssmedia_rule.into(), ctor_cssmedia_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_csssupports_rule) = tmpl_csssupports_rule.get_function(scope) {
        let name_csssupports_rule = v8::String::new(scope, "CSSSupportsRule").unwrap();
        global.define_own_property(scope, name_csssupports_rule.into(), ctor_csssupports_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssmath_clamp) = tmpl_cssmath_clamp.get_function(scope) {
        let name_cssmath_clamp = v8::String::new(scope, "CSSMathClamp").unwrap();
        global.define_own_property(scope, name_cssmath_clamp.into(), ctor_cssmath_clamp.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssmath_invert) = tmpl_cssmath_invert.get_function(scope) {
        let name_cssmath_invert = v8::String::new(scope, "CSSMathInvert").unwrap();
        global.define_own_property(scope, name_cssmath_invert.into(), ctor_cssmath_invert.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssmath_max) = tmpl_cssmath_max.get_function(scope) {
        let name_cssmath_max = v8::String::new(scope, "CSSMathMax").unwrap();
        global.define_own_property(scope, name_cssmath_max.into(), ctor_cssmath_max.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssmath_min) = tmpl_cssmath_min.get_function(scope) {
        let name_cssmath_min = v8::String::new(scope, "CSSMathMin").unwrap();
        global.define_own_property(scope, name_cssmath_min.into(), ctor_cssmath_min.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssmath_negate) = tmpl_cssmath_negate.get_function(scope) {
        let name_cssmath_negate = v8::String::new(scope, "CSSMathNegate").unwrap();
        global.define_own_property(scope, name_cssmath_negate.into(), ctor_cssmath_negate.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssmath_product) = tmpl_cssmath_product.get_function(scope) {
        let name_cssmath_product = v8::String::new(scope, "CSSMathProduct").unwrap();
        global.define_own_property(scope, name_cssmath_product.into(), ctor_cssmath_product.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssmath_sum) = tmpl_cssmath_sum.get_function(scope) {
        let name_cssmath_sum = v8::String::new(scope, "CSSMathSum").unwrap();
        global.define_own_property(scope, name_cssmath_sum.into(), ctor_cssmath_sum.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_background_fetch_update_uievent) = tmpl_background_fetch_update_uievent.get_function(scope) {
        let name_background_fetch_update_uievent = v8::String::new(scope, "BackgroundFetchUpdateUIEvent").unwrap();
        global.define_own_property(scope, name_background_fetch_update_uievent.into(), ctor_background_fetch_update_uievent.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_drag_event) = tmpl_drag_event.get_function(scope) {
        let name_drag_event = v8::String::new(scope, "DragEvent").unwrap();
        global.define_own_property(scope, name_drag_event.into(), ctor_drag_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_pointer_event) = tmpl_pointer_event.get_function(scope) {
        let name_pointer_event = v8::String::new(scope, "PointerEvent").unwrap();
        global.define_own_property(scope, name_pointer_event.into(), ctor_pointer_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_wheel_event) = tmpl_wheel_event.get_function(scope) {
        let name_wheel_event = v8::String::new(scope, "WheelEvent").unwrap();
        global.define_own_property(scope, name_wheel_event.into(), ctor_wheel_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_audio_buffer_source_node) = tmpl_audio_buffer_source_node.get_function(scope) {
        let name_audio_buffer_source_node = v8::String::new(scope, "AudioBufferSourceNode").unwrap();
        global.define_own_property(scope, name_audio_buffer_source_node.into(), ctor_audio_buffer_source_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_constant_source_node) = tmpl_constant_source_node.get_function(scope) {
        let name_constant_source_node = v8::String::new(scope, "ConstantSourceNode").unwrap();
        global.define_own_property(scope, name_constant_source_node.into(), ctor_constant_source_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_oscillator_node) = tmpl_oscillator_node.get_function(scope) {
        let name_oscillator_node = v8::String::new(scope, "OscillatorNode").unwrap();
        global.define_own_property(scope, name_oscillator_node.into(), ctor_oscillator_node.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_webkit_audio_context) = tmpl_webkit_audio_context.get_function(scope) {
        let name_webkit_audio_context = v8::String::new(scope, "webkitAudioContext").unwrap();
        global.define_own_property(scope, name_webkit_audio_context.into(), ctor_webkit_audio_context.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_comment) = tmpl_comment.get_function(scope) {
        let name_comment = v8::String::new(scope, "Comment").unwrap();
        global.define_own_property(scope, name_comment.into(), ctor_comment.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_processing_instruction) = tmpl_processing_instruction.get_function(scope) {
        let name_processing_instruction = v8::String::new(scope, "ProcessingInstruction").unwrap();
        global.define_own_property(scope, name_processing_instruction.into(), ctor_processing_instruction.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_text) = tmpl_text.get_function(scope) {
        let name_text = v8::String::new(scope, "Text").unwrap();
        global.define_own_property(scope, name_text.into(), ctor_text.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xmldocument) = tmpl_xmldocument.get_function(scope) {
        let name_xmldocument = v8::String::new(scope, "XMLDocument").unwrap();
        global.define_own_property(scope, name_xmldocument.into(), ctor_xmldocument.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_shadow_root) = tmpl_shadow_root.get_function(scope) {
        let name_shadow_root = v8::String::new(scope, "ShadowRoot").unwrap();
        global.define_own_property(scope, name_shadow_root.into(), ctor_shadow_root.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlelement) = tmpl_htmlelement.get_function(scope) {
        let name_htmlelement = v8::String::new(scope, "HTMLElement").unwrap();
        global.define_own_property(scope, name_htmlelement.into(), ctor_htmlelement.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_math_mlelement) = tmpl_math_mlelement.get_function(scope) {
        let name_math_mlelement = v8::String::new(scope, "MathMLElement").unwrap();
        global.define_own_property(scope, name_math_mlelement.into(), ctor_math_mlelement.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgelement) = tmpl_svgelement.get_function(scope) {
        let name_svgelement = v8::String::new(scope, "SVGElement").unwrap();
        global.define_own_property(scope, name_svgelement.into(), ctor_svgelement.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gravity_sensor) = tmpl_gravity_sensor.get_function(scope) {
        let name_gravity_sensor = v8::String::new(scope, "GravitySensor").unwrap();
        global.define_own_property(scope, name_gravity_sensor.into(), ctor_gravity_sensor.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_linear_acceleration_sensor) = tmpl_linear_acceleration_sensor.get_function(scope) {
        let name_linear_acceleration_sensor = v8::String::new(scope, "LinearAccelerationSensor").unwrap();
        global.define_own_property(scope, name_linear_acceleration_sensor.into(), ctor_linear_acceleration_sensor.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_absolute_orientation_sensor) = tmpl_absolute_orientation_sensor.get_function(scope) {
        let name_absolute_orientation_sensor = v8::String::new(scope, "AbsoluteOrientationSensor").unwrap();
        global.define_own_property(scope, name_absolute_orientation_sensor.into(), ctor_absolute_orientation_sensor.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_relative_orientation_sensor) = tmpl_relative_orientation_sensor.get_function(scope) {
        let name_relative_orientation_sensor = v8::String::new(scope, "RelativeOrientationSensor").unwrap();
        global.define_own_property(scope, name_relative_orientation_sensor.into(), ctor_relative_orientation_sensor.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrcube_layer) = tmpl_xrcube_layer.get_function(scope) {
        let name_xrcube_layer = v8::String::new(scope, "XRCubeLayer").unwrap();
        global.define_own_property(scope, name_xrcube_layer.into(), ctor_xrcube_layer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrcylinder_layer) = tmpl_xrcylinder_layer.get_function(scope) {
        let name_xrcylinder_layer = v8::String::new(scope, "XRCylinderLayer").unwrap();
        global.define_own_property(scope, name_xrcylinder_layer.into(), ctor_xrcylinder_layer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrequirect_layer) = tmpl_xrequirect_layer.get_function(scope) {
        let name_xrequirect_layer = v8::String::new(scope, "XREquirectLayer").unwrap();
        global.define_own_property(scope, name_xrequirect_layer.into(), ctor_xrequirect_layer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrprojection_layer) = tmpl_xrprojection_layer.get_function(scope) {
        let name_xrprojection_layer = v8::String::new(scope, "XRProjectionLayer").unwrap();
        global.define_own_property(scope, name_xrprojection_layer.into(), ctor_xrprojection_layer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrquad_layer) = tmpl_xrquad_layer.get_function(scope) {
        let name_xrquad_layer = v8::String::new(scope, "XRQuadLayer").unwrap();
        global.define_own_property(scope, name_xrquad_layer.into(), ctor_xrquad_layer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xrbounded_reference_space) = tmpl_xrbounded_reference_space.get_function(scope) {
        let name_xrbounded_reference_space = v8::String::new(scope, "XRBoundedReferenceSpace").unwrap();
        global.define_own_property(scope, name_xrbounded_reference_space.into(), ctor_xrbounded_reference_space.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cdatasection) = tmpl_cdatasection.get_function(scope) {
        let name_cdatasection = v8::String::new(scope, "CDATASection").unwrap();
        global.define_own_property(scope, name_cdatasection.into(), ctor_cdatasection.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svguse_element_shadow_root) = tmpl_svguse_element_shadow_root.get_function(scope) {
        let name_svguse_element_shadow_root = v8::String::new(scope, "SVGUseElementShadowRoot").unwrap();
        global.define_own_property(scope, name_svguse_element_shadow_root.into(), ctor_svguse_element_shadow_root.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlanchor_element) = tmpl_htmlanchor_element.get_function(scope) {
        let name_htmlanchor_element = v8::String::new(scope, "HTMLAnchorElement").unwrap();
        global.define_own_property(scope, name_htmlanchor_element.into(), ctor_htmlanchor_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlarea_element) = tmpl_htmlarea_element.get_function(scope) {
        let name_htmlarea_element = v8::String::new(scope, "HTMLAreaElement").unwrap();
        global.define_own_property(scope, name_htmlarea_element.into(), ctor_htmlarea_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlbrelement) = tmpl_htmlbrelement.get_function(scope) {
        let name_htmlbrelement = v8::String::new(scope, "HTMLBRElement").unwrap();
        global.define_own_property(scope, name_htmlbrelement.into(), ctor_htmlbrelement.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlbase_element) = tmpl_htmlbase_element.get_function(scope) {
        let name_htmlbase_element = v8::String::new(scope, "HTMLBaseElement").unwrap();
        global.define_own_property(scope, name_htmlbase_element.into(), ctor_htmlbase_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlbody_element) = tmpl_htmlbody_element.get_function(scope) {
        let name_htmlbody_element = v8::String::new(scope, "HTMLBodyElement").unwrap();
        global.define_own_property(scope, name_htmlbody_element.into(), ctor_htmlbody_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlbutton_element) = tmpl_htmlbutton_element.get_function(scope) {
        let name_htmlbutton_element = v8::String::new(scope, "HTMLButtonElement").unwrap();
        global.define_own_property(scope, name_htmlbutton_element.into(), ctor_htmlbutton_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlcanvas_element) = tmpl_htmlcanvas_element.get_function(scope) {
        let name_htmlcanvas_element = v8::String::new(scope, "HTMLCanvasElement").unwrap();
        global.define_own_property(scope, name_htmlcanvas_element.into(), ctor_htmlcanvas_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmldlist_element) = tmpl_htmldlist_element.get_function(scope) {
        let name_htmldlist_element = v8::String::new(scope, "HTMLDListElement").unwrap();
        global.define_own_property(scope, name_htmldlist_element.into(), ctor_htmldlist_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmldata_element) = tmpl_htmldata_element.get_function(scope) {
        let name_htmldata_element = v8::String::new(scope, "HTMLDataElement").unwrap();
        global.define_own_property(scope, name_htmldata_element.into(), ctor_htmldata_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmldata_list_element) = tmpl_htmldata_list_element.get_function(scope) {
        let name_htmldata_list_element = v8::String::new(scope, "HTMLDataListElement").unwrap();
        global.define_own_property(scope, name_htmldata_list_element.into(), ctor_htmldata_list_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmldetails_element) = tmpl_htmldetails_element.get_function(scope) {
        let name_htmldetails_element = v8::String::new(scope, "HTMLDetailsElement").unwrap();
        global.define_own_property(scope, name_htmldetails_element.into(), ctor_htmldetails_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmldialog_element) = tmpl_htmldialog_element.get_function(scope) {
        let name_htmldialog_element = v8::String::new(scope, "HTMLDialogElement").unwrap();
        global.define_own_property(scope, name_htmldialog_element.into(), ctor_htmldialog_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmldirectory_element) = tmpl_htmldirectory_element.get_function(scope) {
        let name_htmldirectory_element = v8::String::new(scope, "HTMLDirectoryElement").unwrap();
        global.define_own_property(scope, name_htmldirectory_element.into(), ctor_htmldirectory_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmldiv_element) = tmpl_htmldiv_element.get_function(scope) {
        let name_htmldiv_element = v8::String::new(scope, "HTMLDivElement").unwrap();
        global.define_own_property(scope, name_htmldiv_element.into(), ctor_htmldiv_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlembed_element) = tmpl_htmlembed_element.get_function(scope) {
        let name_htmlembed_element = v8::String::new(scope, "HTMLEmbedElement").unwrap();
        global.define_own_property(scope, name_htmlembed_element.into(), ctor_htmlembed_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlfenced_frame_element) = tmpl_htmlfenced_frame_element.get_function(scope) {
        let name_htmlfenced_frame_element = v8::String::new(scope, "HTMLFencedFrameElement").unwrap();
        global.define_own_property(scope, name_htmlfenced_frame_element.into(), ctor_htmlfenced_frame_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlfield_set_element) = tmpl_htmlfield_set_element.get_function(scope) {
        let name_htmlfield_set_element = v8::String::new(scope, "HTMLFieldSetElement").unwrap();
        global.define_own_property(scope, name_htmlfield_set_element.into(), ctor_htmlfield_set_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlfont_element) = tmpl_htmlfont_element.get_function(scope) {
        let name_htmlfont_element = v8::String::new(scope, "HTMLFontElement").unwrap();
        global.define_own_property(scope, name_htmlfont_element.into(), ctor_htmlfont_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlform_element) = tmpl_htmlform_element.get_function(scope) {
        let name_htmlform_element = v8::String::new(scope, "HTMLFormElement").unwrap();
        global.define_own_property(scope, name_htmlform_element.into(), ctor_htmlform_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlframe_element) = tmpl_htmlframe_element.get_function(scope) {
        let name_htmlframe_element = v8::String::new(scope, "HTMLFrameElement").unwrap();
        global.define_own_property(scope, name_htmlframe_element.into(), ctor_htmlframe_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlframe_set_element) = tmpl_htmlframe_set_element.get_function(scope) {
        let name_htmlframe_set_element = v8::String::new(scope, "HTMLFrameSetElement").unwrap();
        global.define_own_property(scope, name_htmlframe_set_element.into(), ctor_htmlframe_set_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlgeolocation_element) = tmpl_htmlgeolocation_element.get_function(scope) {
        let name_htmlgeolocation_element = v8::String::new(scope, "HTMLGeolocationElement").unwrap();
        global.define_own_property(scope, name_htmlgeolocation_element.into(), ctor_htmlgeolocation_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlhrelement) = tmpl_htmlhrelement.get_function(scope) {
        let name_htmlhrelement = v8::String::new(scope, "HTMLHRElement").unwrap();
        global.define_own_property(scope, name_htmlhrelement.into(), ctor_htmlhrelement.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlhead_element) = tmpl_htmlhead_element.get_function(scope) {
        let name_htmlhead_element = v8::String::new(scope, "HTMLHeadElement").unwrap();
        global.define_own_property(scope, name_htmlhead_element.into(), ctor_htmlhead_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlheading_element) = tmpl_htmlheading_element.get_function(scope) {
        let name_htmlheading_element = v8::String::new(scope, "HTMLHeadingElement").unwrap();
        global.define_own_property(scope, name_htmlheading_element.into(), ctor_htmlheading_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlhtml_element) = tmpl_htmlhtml_element.get_function(scope) {
        let name_htmlhtml_element = v8::String::new(scope, "HTMLHtmlElement").unwrap();
        global.define_own_property(scope, name_htmlhtml_element.into(), ctor_htmlhtml_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmliframe_element) = tmpl_htmliframe_element.get_function(scope) {
        let name_htmliframe_element = v8::String::new(scope, "HTMLIFrameElement").unwrap();
        global.define_own_property(scope, name_htmliframe_element.into(), ctor_htmliframe_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlimage_element) = tmpl_htmlimage_element.get_function(scope) {
        let name_htmlimage_element = v8::String::new(scope, "HTMLImageElement").unwrap();
        global.define_own_property(scope, name_htmlimage_element.into(), ctor_htmlimage_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlinput_element) = tmpl_htmlinput_element.get_function(scope) {
        let name_htmlinput_element = v8::String::new(scope, "HTMLInputElement").unwrap();
        global.define_own_property(scope, name_htmlinput_element.into(), ctor_htmlinput_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmllielement) = tmpl_htmllielement.get_function(scope) {
        let name_htmllielement = v8::String::new(scope, "HTMLLIElement").unwrap();
        global.define_own_property(scope, name_htmllielement.into(), ctor_htmllielement.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmllabel_element) = tmpl_htmllabel_element.get_function(scope) {
        let name_htmllabel_element = v8::String::new(scope, "HTMLLabelElement").unwrap();
        global.define_own_property(scope, name_htmllabel_element.into(), ctor_htmllabel_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmllegend_element) = tmpl_htmllegend_element.get_function(scope) {
        let name_htmllegend_element = v8::String::new(scope, "HTMLLegendElement").unwrap();
        global.define_own_property(scope, name_htmllegend_element.into(), ctor_htmllegend_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmllink_element) = tmpl_htmllink_element.get_function(scope) {
        let name_htmllink_element = v8::String::new(scope, "HTMLLinkElement").unwrap();
        global.define_own_property(scope, name_htmllink_element.into(), ctor_htmllink_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlmap_element) = tmpl_htmlmap_element.get_function(scope) {
        let name_htmlmap_element = v8::String::new(scope, "HTMLMapElement").unwrap();
        global.define_own_property(scope, name_htmlmap_element.into(), ctor_htmlmap_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlmarquee_element) = tmpl_htmlmarquee_element.get_function(scope) {
        let name_htmlmarquee_element = v8::String::new(scope, "HTMLMarqueeElement").unwrap();
        global.define_own_property(scope, name_htmlmarquee_element.into(), ctor_htmlmarquee_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlmedia_element) = tmpl_htmlmedia_element.get_function(scope) {
        let name_htmlmedia_element = v8::String::new(scope, "HTMLMediaElement").unwrap();
        global.define_own_property(scope, name_htmlmedia_element.into(), ctor_htmlmedia_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlmenu_element) = tmpl_htmlmenu_element.get_function(scope) {
        let name_htmlmenu_element = v8::String::new(scope, "HTMLMenuElement").unwrap();
        global.define_own_property(scope, name_htmlmenu_element.into(), ctor_htmlmenu_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlmeta_element) = tmpl_htmlmeta_element.get_function(scope) {
        let name_htmlmeta_element = v8::String::new(scope, "HTMLMetaElement").unwrap();
        global.define_own_property(scope, name_htmlmeta_element.into(), ctor_htmlmeta_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlmeter_element) = tmpl_htmlmeter_element.get_function(scope) {
        let name_htmlmeter_element = v8::String::new(scope, "HTMLMeterElement").unwrap();
        global.define_own_property(scope, name_htmlmeter_element.into(), ctor_htmlmeter_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlmod_element) = tmpl_htmlmod_element.get_function(scope) {
        let name_htmlmod_element = v8::String::new(scope, "HTMLModElement").unwrap();
        global.define_own_property(scope, name_htmlmod_element.into(), ctor_htmlmod_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlmodel_element) = tmpl_htmlmodel_element.get_function(scope) {
        let name_htmlmodel_element = v8::String::new(scope, "HTMLModelElement").unwrap();
        global.define_own_property(scope, name_htmlmodel_element.into(), ctor_htmlmodel_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlolist_element) = tmpl_htmlolist_element.get_function(scope) {
        let name_htmlolist_element = v8::String::new(scope, "HTMLOListElement").unwrap();
        global.define_own_property(scope, name_htmlolist_element.into(), ctor_htmlolist_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlobject_element) = tmpl_htmlobject_element.get_function(scope) {
        let name_htmlobject_element = v8::String::new(scope, "HTMLObjectElement").unwrap();
        global.define_own_property(scope, name_htmlobject_element.into(), ctor_htmlobject_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlopt_group_element) = tmpl_htmlopt_group_element.get_function(scope) {
        let name_htmlopt_group_element = v8::String::new(scope, "HTMLOptGroupElement").unwrap();
        global.define_own_property(scope, name_htmlopt_group_element.into(), ctor_htmlopt_group_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmloption_element) = tmpl_htmloption_element.get_function(scope) {
        let name_htmloption_element = v8::String::new(scope, "HTMLOptionElement").unwrap();
        global.define_own_property(scope, name_htmloption_element.into(), ctor_htmloption_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmloutput_element) = tmpl_htmloutput_element.get_function(scope) {
        let name_htmloutput_element = v8::String::new(scope, "HTMLOutputElement").unwrap();
        global.define_own_property(scope, name_htmloutput_element.into(), ctor_htmloutput_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlparagraph_element) = tmpl_htmlparagraph_element.get_function(scope) {
        let name_htmlparagraph_element = v8::String::new(scope, "HTMLParagraphElement").unwrap();
        global.define_own_property(scope, name_htmlparagraph_element.into(), ctor_htmlparagraph_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlparam_element) = tmpl_htmlparam_element.get_function(scope) {
        let name_htmlparam_element = v8::String::new(scope, "HTMLParamElement").unwrap();
        global.define_own_property(scope, name_htmlparam_element.into(), ctor_htmlparam_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlpicture_element) = tmpl_htmlpicture_element.get_function(scope) {
        let name_htmlpicture_element = v8::String::new(scope, "HTMLPictureElement").unwrap();
        global.define_own_property(scope, name_htmlpicture_element.into(), ctor_htmlpicture_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlportal_element) = tmpl_htmlportal_element.get_function(scope) {
        let name_htmlportal_element = v8::String::new(scope, "HTMLPortalElement").unwrap();
        global.define_own_property(scope, name_htmlportal_element.into(), ctor_htmlportal_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlpre_element) = tmpl_htmlpre_element.get_function(scope) {
        let name_htmlpre_element = v8::String::new(scope, "HTMLPreElement").unwrap();
        global.define_own_property(scope, name_htmlpre_element.into(), ctor_htmlpre_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlprogress_element) = tmpl_htmlprogress_element.get_function(scope) {
        let name_htmlprogress_element = v8::String::new(scope, "HTMLProgressElement").unwrap();
        global.define_own_property(scope, name_htmlprogress_element.into(), ctor_htmlprogress_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlquote_element) = tmpl_htmlquote_element.get_function(scope) {
        let name_htmlquote_element = v8::String::new(scope, "HTMLQuoteElement").unwrap();
        global.define_own_property(scope, name_htmlquote_element.into(), ctor_htmlquote_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlscript_element) = tmpl_htmlscript_element.get_function(scope) {
        let name_htmlscript_element = v8::String::new(scope, "HTMLScriptElement").unwrap();
        global.define_own_property(scope, name_htmlscript_element.into(), ctor_htmlscript_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlselect_element) = tmpl_htmlselect_element.get_function(scope) {
        let name_htmlselect_element = v8::String::new(scope, "HTMLSelectElement").unwrap();
        global.define_own_property(scope, name_htmlselect_element.into(), ctor_htmlselect_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlselected_content_element) = tmpl_htmlselected_content_element.get_function(scope) {
        let name_htmlselected_content_element = v8::String::new(scope, "HTMLSelectedContentElement").unwrap();
        global.define_own_property(scope, name_htmlselected_content_element.into(), ctor_htmlselected_content_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlslot_element) = tmpl_htmlslot_element.get_function(scope) {
        let name_htmlslot_element = v8::String::new(scope, "HTMLSlotElement").unwrap();
        global.define_own_property(scope, name_htmlslot_element.into(), ctor_htmlslot_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlsource_element) = tmpl_htmlsource_element.get_function(scope) {
        let name_htmlsource_element = v8::String::new(scope, "HTMLSourceElement").unwrap();
        global.define_own_property(scope, name_htmlsource_element.into(), ctor_htmlsource_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlspan_element) = tmpl_htmlspan_element.get_function(scope) {
        let name_htmlspan_element = v8::String::new(scope, "HTMLSpanElement").unwrap();
        global.define_own_property(scope, name_htmlspan_element.into(), ctor_htmlspan_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlstyle_element) = tmpl_htmlstyle_element.get_function(scope) {
        let name_htmlstyle_element = v8::String::new(scope, "HTMLStyleElement").unwrap();
        global.define_own_property(scope, name_htmlstyle_element.into(), ctor_htmlstyle_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmltable_caption_element) = tmpl_htmltable_caption_element.get_function(scope) {
        let name_htmltable_caption_element = v8::String::new(scope, "HTMLTableCaptionElement").unwrap();
        global.define_own_property(scope, name_htmltable_caption_element.into(), ctor_htmltable_caption_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmltable_cell_element) = tmpl_htmltable_cell_element.get_function(scope) {
        let name_htmltable_cell_element = v8::String::new(scope, "HTMLTableCellElement").unwrap();
        global.define_own_property(scope, name_htmltable_cell_element.into(), ctor_htmltable_cell_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmltable_col_element) = tmpl_htmltable_col_element.get_function(scope) {
        let name_htmltable_col_element = v8::String::new(scope, "HTMLTableColElement").unwrap();
        global.define_own_property(scope, name_htmltable_col_element.into(), ctor_htmltable_col_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmltable_element) = tmpl_htmltable_element.get_function(scope) {
        let name_htmltable_element = v8::String::new(scope, "HTMLTableElement").unwrap();
        global.define_own_property(scope, name_htmltable_element.into(), ctor_htmltable_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmltable_row_element) = tmpl_htmltable_row_element.get_function(scope) {
        let name_htmltable_row_element = v8::String::new(scope, "HTMLTableRowElement").unwrap();
        global.define_own_property(scope, name_htmltable_row_element.into(), ctor_htmltable_row_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmltable_section_element) = tmpl_htmltable_section_element.get_function(scope) {
        let name_htmltable_section_element = v8::String::new(scope, "HTMLTableSectionElement").unwrap();
        global.define_own_property(scope, name_htmltable_section_element.into(), ctor_htmltable_section_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmltemplate_element) = tmpl_htmltemplate_element.get_function(scope) {
        let name_htmltemplate_element = v8::String::new(scope, "HTMLTemplateElement").unwrap();
        global.define_own_property(scope, name_htmltemplate_element.into(), ctor_htmltemplate_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmltext_area_element) = tmpl_htmltext_area_element.get_function(scope) {
        let name_htmltext_area_element = v8::String::new(scope, "HTMLTextAreaElement").unwrap();
        global.define_own_property(scope, name_htmltext_area_element.into(), ctor_htmltext_area_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmltime_element) = tmpl_htmltime_element.get_function(scope) {
        let name_htmltime_element = v8::String::new(scope, "HTMLTimeElement").unwrap();
        global.define_own_property(scope, name_htmltime_element.into(), ctor_htmltime_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmltitle_element) = tmpl_htmltitle_element.get_function(scope) {
        let name_htmltitle_element = v8::String::new(scope, "HTMLTitleElement").unwrap();
        global.define_own_property(scope, name_htmltitle_element.into(), ctor_htmltitle_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmltrack_element) = tmpl_htmltrack_element.get_function(scope) {
        let name_htmltrack_element = v8::String::new(scope, "HTMLTrackElement").unwrap();
        global.define_own_property(scope, name_htmltrack_element.into(), ctor_htmltrack_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlulist_element) = tmpl_htmlulist_element.get_function(scope) {
        let name_htmlulist_element = v8::String::new(scope, "HTMLUListElement").unwrap();
        global.define_own_property(scope, name_htmlulist_element.into(), ctor_htmlulist_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlunknown_element) = tmpl_htmlunknown_element.get_function(scope) {
        let name_htmlunknown_element = v8::String::new(scope, "HTMLUnknownElement").unwrap();
        global.define_own_property(scope, name_htmlunknown_element.into(), ctor_htmlunknown_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_math_mlanchor_element) = tmpl_math_mlanchor_element.get_function(scope) {
        let name_math_mlanchor_element = v8::String::new(scope, "MathMLAnchorElement").unwrap();
        global.define_own_property(scope, name_math_mlanchor_element.into(), ctor_math_mlanchor_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svganimation_element) = tmpl_svganimation_element.get_function(scope) {
        let name_svganimation_element = v8::String::new(scope, "SVGAnimationElement").unwrap();
        global.define_own_property(scope, name_svganimation_element.into(), ctor_svganimation_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgclip_path_element) = tmpl_svgclip_path_element.get_function(scope) {
        let name_svgclip_path_element = v8::String::new(scope, "SVGClipPathElement").unwrap();
        global.define_own_property(scope, name_svgclip_path_element.into(), ctor_svgclip_path_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgcomponent_transfer_function_element) = tmpl_svgcomponent_transfer_function_element.get_function(scope) {
        let name_svgcomponent_transfer_function_element = v8::String::new(scope, "SVGComponentTransferFunctionElement").unwrap();
        global.define_own_property(scope, name_svgcomponent_transfer_function_element.into(), ctor_svgcomponent_transfer_function_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgdesc_element) = tmpl_svgdesc_element.get_function(scope) {
        let name_svgdesc_element = v8::String::new(scope, "SVGDescElement").unwrap();
        global.define_own_property(scope, name_svgdesc_element.into(), ctor_svgdesc_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfeblend_element) = tmpl_svgfeblend_element.get_function(scope) {
        let name_svgfeblend_element = v8::String::new(scope, "SVGFEBlendElement").unwrap();
        global.define_own_property(scope, name_svgfeblend_element.into(), ctor_svgfeblend_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfecolor_matrix_element) = tmpl_svgfecolor_matrix_element.get_function(scope) {
        let name_svgfecolor_matrix_element = v8::String::new(scope, "SVGFEColorMatrixElement").unwrap();
        global.define_own_property(scope, name_svgfecolor_matrix_element.into(), ctor_svgfecolor_matrix_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfecomponent_transfer_element) = tmpl_svgfecomponent_transfer_element.get_function(scope) {
        let name_svgfecomponent_transfer_element = v8::String::new(scope, "SVGFEComponentTransferElement").unwrap();
        global.define_own_property(scope, name_svgfecomponent_transfer_element.into(), ctor_svgfecomponent_transfer_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfecomposite_element) = tmpl_svgfecomposite_element.get_function(scope) {
        let name_svgfecomposite_element = v8::String::new(scope, "SVGFECompositeElement").unwrap();
        global.define_own_property(scope, name_svgfecomposite_element.into(), ctor_svgfecomposite_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfeconvolve_matrix_element) = tmpl_svgfeconvolve_matrix_element.get_function(scope) {
        let name_svgfeconvolve_matrix_element = v8::String::new(scope, "SVGFEConvolveMatrixElement").unwrap();
        global.define_own_property(scope, name_svgfeconvolve_matrix_element.into(), ctor_svgfeconvolve_matrix_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfediffuse_lighting_element) = tmpl_svgfediffuse_lighting_element.get_function(scope) {
        let name_svgfediffuse_lighting_element = v8::String::new(scope, "SVGFEDiffuseLightingElement").unwrap();
        global.define_own_property(scope, name_svgfediffuse_lighting_element.into(), ctor_svgfediffuse_lighting_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfedisplacement_map_element) = tmpl_svgfedisplacement_map_element.get_function(scope) {
        let name_svgfedisplacement_map_element = v8::String::new(scope, "SVGFEDisplacementMapElement").unwrap();
        global.define_own_property(scope, name_svgfedisplacement_map_element.into(), ctor_svgfedisplacement_map_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfedistant_light_element) = tmpl_svgfedistant_light_element.get_function(scope) {
        let name_svgfedistant_light_element = v8::String::new(scope, "SVGFEDistantLightElement").unwrap();
        global.define_own_property(scope, name_svgfedistant_light_element.into(), ctor_svgfedistant_light_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfedrop_shadow_element) = tmpl_svgfedrop_shadow_element.get_function(scope) {
        let name_svgfedrop_shadow_element = v8::String::new(scope, "SVGFEDropShadowElement").unwrap();
        global.define_own_property(scope, name_svgfedrop_shadow_element.into(), ctor_svgfedrop_shadow_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfeflood_element) = tmpl_svgfeflood_element.get_function(scope) {
        let name_svgfeflood_element = v8::String::new(scope, "SVGFEFloodElement").unwrap();
        global.define_own_property(scope, name_svgfeflood_element.into(), ctor_svgfeflood_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfegaussian_blur_element) = tmpl_svgfegaussian_blur_element.get_function(scope) {
        let name_svgfegaussian_blur_element = v8::String::new(scope, "SVGFEGaussianBlurElement").unwrap();
        global.define_own_property(scope, name_svgfegaussian_blur_element.into(), ctor_svgfegaussian_blur_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfeimage_element) = tmpl_svgfeimage_element.get_function(scope) {
        let name_svgfeimage_element = v8::String::new(scope, "SVGFEImageElement").unwrap();
        global.define_own_property(scope, name_svgfeimage_element.into(), ctor_svgfeimage_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfemerge_element) = tmpl_svgfemerge_element.get_function(scope) {
        let name_svgfemerge_element = v8::String::new(scope, "SVGFEMergeElement").unwrap();
        global.define_own_property(scope, name_svgfemerge_element.into(), ctor_svgfemerge_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfemerge_node_element) = tmpl_svgfemerge_node_element.get_function(scope) {
        let name_svgfemerge_node_element = v8::String::new(scope, "SVGFEMergeNodeElement").unwrap();
        global.define_own_property(scope, name_svgfemerge_node_element.into(), ctor_svgfemerge_node_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfemorphology_element) = tmpl_svgfemorphology_element.get_function(scope) {
        let name_svgfemorphology_element = v8::String::new(scope, "SVGFEMorphologyElement").unwrap();
        global.define_own_property(scope, name_svgfemorphology_element.into(), ctor_svgfemorphology_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfeoffset_element) = tmpl_svgfeoffset_element.get_function(scope) {
        let name_svgfeoffset_element = v8::String::new(scope, "SVGFEOffsetElement").unwrap();
        global.define_own_property(scope, name_svgfeoffset_element.into(), ctor_svgfeoffset_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfepoint_light_element) = tmpl_svgfepoint_light_element.get_function(scope) {
        let name_svgfepoint_light_element = v8::String::new(scope, "SVGFEPointLightElement").unwrap();
        global.define_own_property(scope, name_svgfepoint_light_element.into(), ctor_svgfepoint_light_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfespecular_lighting_element) = tmpl_svgfespecular_lighting_element.get_function(scope) {
        let name_svgfespecular_lighting_element = v8::String::new(scope, "SVGFESpecularLightingElement").unwrap();
        global.define_own_property(scope, name_svgfespecular_lighting_element.into(), ctor_svgfespecular_lighting_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfespot_light_element) = tmpl_svgfespot_light_element.get_function(scope) {
        let name_svgfespot_light_element = v8::String::new(scope, "SVGFESpotLightElement").unwrap();
        global.define_own_property(scope, name_svgfespot_light_element.into(), ctor_svgfespot_light_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfetile_element) = tmpl_svgfetile_element.get_function(scope) {
        let name_svgfetile_element = v8::String::new(scope, "SVGFETileElement").unwrap();
        global.define_own_property(scope, name_svgfetile_element.into(), ctor_svgfetile_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfeturbulence_element) = tmpl_svgfeturbulence_element.get_function(scope) {
        let name_svgfeturbulence_element = v8::String::new(scope, "SVGFETurbulenceElement").unwrap();
        global.define_own_property(scope, name_svgfeturbulence_element.into(), ctor_svgfeturbulence_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfilter_element) = tmpl_svgfilter_element.get_function(scope) {
        let name_svgfilter_element = v8::String::new(scope, "SVGFilterElement").unwrap();
        global.define_own_property(scope, name_svgfilter_element.into(), ctor_svgfilter_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svggradient_element) = tmpl_svggradient_element.get_function(scope) {
        let name_svggradient_element = v8::String::new(scope, "SVGGradientElement").unwrap();
        global.define_own_property(scope, name_svggradient_element.into(), ctor_svggradient_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svggraphics_element) = tmpl_svggraphics_element.get_function(scope) {
        let name_svggraphics_element = v8::String::new(scope, "SVGGraphicsElement").unwrap();
        global.define_own_property(scope, name_svggraphics_element.into(), ctor_svggraphics_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgmpath_element) = tmpl_svgmpath_element.get_function(scope) {
        let name_svgmpath_element = v8::String::new(scope, "SVGMPathElement").unwrap();
        global.define_own_property(scope, name_svgmpath_element.into(), ctor_svgmpath_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgmarker_element) = tmpl_svgmarker_element.get_function(scope) {
        let name_svgmarker_element = v8::String::new(scope, "SVGMarkerElement").unwrap();
        global.define_own_property(scope, name_svgmarker_element.into(), ctor_svgmarker_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgmask_element) = tmpl_svgmask_element.get_function(scope) {
        let name_svgmask_element = v8::String::new(scope, "SVGMaskElement").unwrap();
        global.define_own_property(scope, name_svgmask_element.into(), ctor_svgmask_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgmetadata_element) = tmpl_svgmetadata_element.get_function(scope) {
        let name_svgmetadata_element = v8::String::new(scope, "SVGMetadataElement").unwrap();
        global.define_own_property(scope, name_svgmetadata_element.into(), ctor_svgmetadata_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgpattern_element) = tmpl_svgpattern_element.get_function(scope) {
        let name_svgpattern_element = v8::String::new(scope, "SVGPatternElement").unwrap();
        global.define_own_property(scope, name_svgpattern_element.into(), ctor_svgpattern_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgscript_element) = tmpl_svgscript_element.get_function(scope) {
        let name_svgscript_element = v8::String::new(scope, "SVGScriptElement").unwrap();
        global.define_own_property(scope, name_svgscript_element.into(), ctor_svgscript_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgstop_element) = tmpl_svgstop_element.get_function(scope) {
        let name_svgstop_element = v8::String::new(scope, "SVGStopElement").unwrap();
        global.define_own_property(scope, name_svgstop_element.into(), ctor_svgstop_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgstyle_element) = tmpl_svgstyle_element.get_function(scope) {
        let name_svgstyle_element = v8::String::new(scope, "SVGStyleElement").unwrap();
        global.define_own_property(scope, name_svgstyle_element.into(), ctor_svgstyle_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgtitle_element) = tmpl_svgtitle_element.get_function(scope) {
        let name_svgtitle_element = v8::String::new(scope, "SVGTitleElement").unwrap();
        global.define_own_property(scope, name_svgtitle_element.into(), ctor_svgtitle_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgview_element) = tmpl_svgview_element.get_function(scope) {
        let name_svgview_element = v8::String::new(scope, "SVGViewElement").unwrap();
        global.define_own_property(scope, name_svgview_element.into(), ctor_svgview_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlaudio_element) = tmpl_htmlaudio_element.get_function(scope) {
        let name_htmlaudio_element = v8::String::new(scope, "HTMLAudioElement").unwrap();
        global.define_own_property(scope, name_htmlaudio_element.into(), ctor_htmlaudio_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlvideo_element) = tmpl_htmlvideo_element.get_function(scope) {
        let name_htmlvideo_element = v8::String::new(scope, "HTMLVideoElement").unwrap();
        global.define_own_property(scope, name_htmlvideo_element.into(), ctor_htmlvideo_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svganimate_element) = tmpl_svganimate_element.get_function(scope) {
        let name_svganimate_element = v8::String::new(scope, "SVGAnimateElement").unwrap();
        global.define_own_property(scope, name_svganimate_element.into(), ctor_svganimate_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svganimate_motion_element) = tmpl_svganimate_motion_element.get_function(scope) {
        let name_svganimate_motion_element = v8::String::new(scope, "SVGAnimateMotionElement").unwrap();
        global.define_own_property(scope, name_svganimate_motion_element.into(), ctor_svganimate_motion_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svganimate_transform_element) = tmpl_svganimate_transform_element.get_function(scope) {
        let name_svganimate_transform_element = v8::String::new(scope, "SVGAnimateTransformElement").unwrap();
        global.define_own_property(scope, name_svganimate_transform_element.into(), ctor_svganimate_transform_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgset_element) = tmpl_svgset_element.get_function(scope) {
        let name_svgset_element = v8::String::new(scope, "SVGSetElement").unwrap();
        global.define_own_property(scope, name_svgset_element.into(), ctor_svgset_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfefunc_aelement) = tmpl_svgfefunc_aelement.get_function(scope) {
        let name_svgfefunc_aelement = v8::String::new(scope, "SVGFEFuncAElement").unwrap();
        global.define_own_property(scope, name_svgfefunc_aelement.into(), ctor_svgfefunc_aelement.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfefunc_belement) = tmpl_svgfefunc_belement.get_function(scope) {
        let name_svgfefunc_belement = v8::String::new(scope, "SVGFEFuncBElement").unwrap();
        global.define_own_property(scope, name_svgfefunc_belement.into(), ctor_svgfefunc_belement.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfefunc_gelement) = tmpl_svgfefunc_gelement.get_function(scope) {
        let name_svgfefunc_gelement = v8::String::new(scope, "SVGFEFuncGElement").unwrap();
        global.define_own_property(scope, name_svgfefunc_gelement.into(), ctor_svgfefunc_gelement.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfefunc_relement) = tmpl_svgfefunc_relement.get_function(scope) {
        let name_svgfefunc_relement = v8::String::new(scope, "SVGFEFuncRElement").unwrap();
        global.define_own_property(scope, name_svgfefunc_relement.into(), ctor_svgfefunc_relement.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svglinear_gradient_element) = tmpl_svglinear_gradient_element.get_function(scope) {
        let name_svglinear_gradient_element = v8::String::new(scope, "SVGLinearGradientElement").unwrap();
        global.define_own_property(scope, name_svglinear_gradient_element.into(), ctor_svglinear_gradient_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgradial_gradient_element) = tmpl_svgradial_gradient_element.get_function(scope) {
        let name_svgradial_gradient_element = v8::String::new(scope, "SVGRadialGradientElement").unwrap();
        global.define_own_property(scope, name_svgradial_gradient_element.into(), ctor_svgradial_gradient_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgaelement) = tmpl_svgaelement.get_function(scope) {
        let name_svgaelement = v8::String::new(scope, "SVGAElement").unwrap();
        global.define_own_property(scope, name_svgaelement.into(), ctor_svgaelement.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgdefs_element) = tmpl_svgdefs_element.get_function(scope) {
        let name_svgdefs_element = v8::String::new(scope, "SVGDefsElement").unwrap();
        global.define_own_property(scope, name_svgdefs_element.into(), ctor_svgdefs_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgforeign_object_element) = tmpl_svgforeign_object_element.get_function(scope) {
        let name_svgforeign_object_element = v8::String::new(scope, "SVGForeignObjectElement").unwrap();
        global.define_own_property(scope, name_svgforeign_object_element.into(), ctor_svgforeign_object_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svggelement) = tmpl_svggelement.get_function(scope) {
        let name_svggelement = v8::String::new(scope, "SVGGElement").unwrap();
        global.define_own_property(scope, name_svggelement.into(), ctor_svggelement.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svggeometry_element) = tmpl_svggeometry_element.get_function(scope) {
        let name_svggeometry_element = v8::String::new(scope, "SVGGeometryElement").unwrap();
        global.define_own_property(scope, name_svggeometry_element.into(), ctor_svggeometry_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgimage_element) = tmpl_svgimage_element.get_function(scope) {
        let name_svgimage_element = v8::String::new(scope, "SVGImageElement").unwrap();
        global.define_own_property(scope, name_svgimage_element.into(), ctor_svgimage_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgsvgelement) = tmpl_svgsvgelement.get_function(scope) {
        let name_svgsvgelement = v8::String::new(scope, "SVGSVGElement").unwrap();
        global.define_own_property(scope, name_svgsvgelement.into(), ctor_svgsvgelement.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgswitch_element) = tmpl_svgswitch_element.get_function(scope) {
        let name_svgswitch_element = v8::String::new(scope, "SVGSwitchElement").unwrap();
        global.define_own_property(scope, name_svgswitch_element.into(), ctor_svgswitch_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgsymbol_element) = tmpl_svgsymbol_element.get_function(scope) {
        let name_svgsymbol_element = v8::String::new(scope, "SVGSymbolElement").unwrap();
        global.define_own_property(scope, name_svgsymbol_element.into(), ctor_svgsymbol_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgtext_content_element) = tmpl_svgtext_content_element.get_function(scope) {
        let name_svgtext_content_element = v8::String::new(scope, "SVGTextContentElement").unwrap();
        global.define_own_property(scope, name_svgtext_content_element.into(), ctor_svgtext_content_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svguse_element) = tmpl_svguse_element.get_function(scope) {
        let name_svguse_element = v8::String::new(scope, "SVGUseElement").unwrap();
        global.define_own_property(scope, name_svguse_element.into(), ctor_svguse_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgcircle_element) = tmpl_svgcircle_element.get_function(scope) {
        let name_svgcircle_element = v8::String::new(scope, "SVGCircleElement").unwrap();
        global.define_own_property(scope, name_svgcircle_element.into(), ctor_svgcircle_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgellipse_element) = tmpl_svgellipse_element.get_function(scope) {
        let name_svgellipse_element = v8::String::new(scope, "SVGEllipseElement").unwrap();
        global.define_own_property(scope, name_svgellipse_element.into(), ctor_svgellipse_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgline_element) = tmpl_svgline_element.get_function(scope) {
        let name_svgline_element = v8::String::new(scope, "SVGLineElement").unwrap();
        global.define_own_property(scope, name_svgline_element.into(), ctor_svgline_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgpath_element) = tmpl_svgpath_element.get_function(scope) {
        let name_svgpath_element = v8::String::new(scope, "SVGPathElement").unwrap();
        global.define_own_property(scope, name_svgpath_element.into(), ctor_svgpath_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgpolygon_element) = tmpl_svgpolygon_element.get_function(scope) {
        let name_svgpolygon_element = v8::String::new(scope, "SVGPolygonElement").unwrap();
        global.define_own_property(scope, name_svgpolygon_element.into(), ctor_svgpolygon_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgpolyline_element) = tmpl_svgpolyline_element.get_function(scope) {
        let name_svgpolyline_element = v8::String::new(scope, "SVGPolylineElement").unwrap();
        global.define_own_property(scope, name_svgpolyline_element.into(), ctor_svgpolyline_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgrect_element) = tmpl_svgrect_element.get_function(scope) {
        let name_svgrect_element = v8::String::new(scope, "SVGRectElement").unwrap();
        global.define_own_property(scope, name_svgrect_element.into(), ctor_svgrect_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgtext_path_element) = tmpl_svgtext_path_element.get_function(scope) {
        let name_svgtext_path_element = v8::String::new(scope, "SVGTextPathElement").unwrap();
        global.define_own_property(scope, name_svgtext_path_element.into(), ctor_svgtext_path_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgtext_positioning_element) = tmpl_svgtext_positioning_element.get_function(scope) {
        let name_svgtext_positioning_element = v8::String::new(scope, "SVGTextPositioningElement").unwrap();
        global.define_own_property(scope, name_svgtext_positioning_element.into(), ctor_svgtext_positioning_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgtspan_element) = tmpl_svgtspan_element.get_function(scope) {
        let name_svgtspan_element = v8::String::new(scope, "SVGTSpanElement").unwrap();
        global.define_own_property(scope, name_svgtspan_element.into(), ctor_svgtspan_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgtext_element) = tmpl_svgtext_element.get_function(scope) {
        let name_svgtext_element = v8::String::new(scope, "SVGTextElement").unwrap();
        global.define_own_property(scope, name_svgtext_element.into(), ctor_svgtext_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
}
