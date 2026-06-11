//! Generated install_all — creates all templates in topological order.

use v8::Local;
use v8::Object;
use v8::FunctionTemplate;

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
    let tmpl_authenticator_response = super::credentials::create_authenticator_response_template(scope, None);
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
    let tmpl_bluetooth_le_scan = super::bluetooth::create_bluetooth_le_scan_template(scope, None);
    templates.insert("BluetoothLEScan", tmpl_bluetooth_le_scan);
    let tmpl_bluetooth_le_scan_filter = super::bluetooth::create_bluetooth_le_scan_filter_template(scope, None);
    templates.insert("BluetoothLEScanFilter", tmpl_bluetooth_le_scan_filter);
    let tmpl_bluetooth_manufacturer_data_filter = super::bluetooth::create_bluetooth_manufacturer_data_filter_template(scope, None);
    templates.insert("BluetoothManufacturerDataFilter", tmpl_bluetooth_manufacturer_data_filter);
    let tmpl_bluetooth_manufacturer_data_map = super::bluetooth::create_bluetooth_manufacturer_data_map_template(scope, None);
    templates.insert("BluetoothManufacturerDataMap", tmpl_bluetooth_manufacturer_data_map);
    let tmpl_bluetooth_remote_gatt_descriptor = super::bluetooth::create_bluetooth_remote_gatt_descriptor_template(scope, None);
    templates.insert("BluetoothRemoteGATTDescriptor", tmpl_bluetooth_remote_gatt_descriptor);
    let tmpl_bluetooth_remote_gatt_server = super::bluetooth::create_bluetooth_remote_gatt_server_template(scope, None);
    templates.insert("BluetoothRemoteGATTServer", tmpl_bluetooth_remote_gatt_server);
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
    let tmpl_css_font_feature_values_map = super::css_om::create_css_font_feature_values_map_template(scope, None);
    templates.insert("CSSFontFeatureValuesMap", tmpl_css_font_feature_values_map);
    let tmpl_css_numeric_array = super::css_om::create_css_numeric_array_template(scope, None);
    templates.insert("CSSNumericArray", tmpl_css_numeric_array);
    let tmpl_css_parser_rule = super::css_om::create_css_parser_rule_template(scope, None);
    templates.insert("CSSParserRule", tmpl_css_parser_rule);
    let tmpl_css_parser_value = super::css_om::create_css_parser_value_template(scope, None);
    templates.insert("CSSParserValue", tmpl_css_parser_value);
    let tmpl_css_pseudo_element = super::css_om::create_css_pseudo_element_template(scope, None);
    templates.insert("CSSPseudoElement", tmpl_css_pseudo_element);
    let tmpl_css_rule = super::css_om::create_css_rule_template(scope, None);
    templates.insert("CSSRule", tmpl_css_rule);
    let tmpl_css_rule_list = super::css_om::create_css_rule_list_template(scope, None);
    templates.insert("CSSRuleList", tmpl_css_rule_list);
    let tmpl_css_style_declaration = super::css_om::create_css_style_declaration_template(scope, None);
    templates.insert("CSSStyleDeclaration", tmpl_css_style_declaration);
    let tmpl_css_style_value = super::css_om::create_css_style_value_template(scope, None);
    templates.insert("CSSStyleValue", tmpl_css_style_value);
    let tmpl_css_transform_component = super::css_om::create_css_transform_component_template(scope, None);
    templates.insert("CSSTransformComponent", tmpl_css_transform_component);
    let tmpl_css_variable_reference_value = super::css_om::create_css_variable_reference_value_template(scope, None);
    templates.insert("CSSVariableReferenceValue", tmpl_css_variable_reference_value);
    let tmpl_cache = super::cache_api::create_cache_template(scope, None);
    templates.insert("Cache", tmpl_cache);
    let tmpl_cache_storage = super::cache_api::create_cache_storage_template(scope, None);
    templates.insert("CacheStorage", tmpl_cache_storage);
    let tmpl_canvas_gradient = super::web_apis::create_canvas_gradient_template(scope, None);
    templates.insert("CanvasGradient", tmpl_canvas_gradient);
    let tmpl_canvas_pattern = super::web_apis::create_canvas_pattern_template(scope, None);
    templates.insert("CanvasPattern", tmpl_canvas_pattern);
    let tmpl_canvas_rendering_context2d = super::web_apis::create_canvas_rendering_context2d_template(scope, None);
    templates.insert("CanvasRenderingContext2D", tmpl_canvas_rendering_context2d);
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
    let tmpl_credential = super::credentials::create_credential_template(scope, None);
    templates.insert("Credential", tmpl_credential);
    let tmpl_credentials_container = super::credentials::create_credentials_container_template(scope, None);
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
    let tmpl_dom_exception = super::web_apis::create_dom_exception_template(scope, None);
    templates.insert("DOMException", tmpl_dom_exception);
    let tmpl_dom_implementation = super::dom_core::create_dom_implementation_template(scope, None);
    templates.insert("DOMImplementation", tmpl_dom_implementation);
    let tmpl_dom_matrix_read_only = super::dom_core::create_dom_matrix_read_only_template(scope, None);
    templates.insert("DOMMatrixReadOnly", tmpl_dom_matrix_read_only);
    let tmpl_dom_parser = super::web_apis::create_dom_parser_template(scope, None);
    templates.insert("DOMParser", tmpl_dom_parser);
    let tmpl_dom_point_read_only = super::dom_core::create_dom_point_read_only_template(scope, None);
    templates.insert("DOMPointReadOnly", tmpl_dom_point_read_only);
    let tmpl_dom_quad = super::dom_core::create_dom_quad_template(scope, None);
    templates.insert("DOMQuad", tmpl_dom_quad);
    let tmpl_dom_rect_list = super::web_apis::create_dom_rect_list_template(scope, None);
    templates.insert("DOMRectList", tmpl_dom_rect_list);
    let tmpl_dom_rect_read_only = super::dom_core::create_dom_rect_read_only_template(scope, None);
    templates.insert("DOMRectReadOnly", tmpl_dom_rect_read_only);
    let tmpl_dom_string_list = super::idb::create_dom_string_list_template(scope, None);
    templates.insert("DOMStringList", tmpl_dom_string_list);
    let tmpl_dom_string_map = super::web_apis::create_dom_string_map_template(scope, None);
    templates.insert("DOMStringMap", tmpl_dom_string_map);
    let tmpl_dom_token_list = super::dom_core::create_dom_token_list_template(scope, None);
    templates.insert("DOMTokenList", tmpl_dom_token_list);
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
    let tmpl_gpu = super::gpu::create_gpu_template(scope, None);
    templates.insert("GPU", tmpl_gpu);
    let tmpl_gpu_adapter = super::gpu::create_gpu_adapter_template(scope, None);
    templates.insert("GPUAdapter", tmpl_gpu_adapter);
    let tmpl_gpu_adapter_info = super::gpu::create_gpu_adapter_info_template(scope, None);
    templates.insert("GPUAdapterInfo", tmpl_gpu_adapter_info);
    let tmpl_gpu_bind_group = super::gpu::create_gpu_bind_group_template(scope, None);
    templates.insert("GPUBindGroup", tmpl_gpu_bind_group);
    let tmpl_gpu_bind_group_layout = super::gpu::create_gpu_bind_group_layout_template(scope, None);
    templates.insert("GPUBindGroupLayout", tmpl_gpu_bind_group_layout);
    let tmpl_gpu_buffer = super::gpu::create_gpu_buffer_template(scope, None);
    templates.insert("GPUBuffer", tmpl_gpu_buffer);
    let tmpl_gpu_canvas_context = super::gpu::create_gpu_canvas_context_template(scope, None);
    templates.insert("GPUCanvasContext", tmpl_gpu_canvas_context);
    let tmpl_gpu_command_buffer = super::gpu::create_gpu_command_buffer_template(scope, None);
    templates.insert("GPUCommandBuffer", tmpl_gpu_command_buffer);
    let tmpl_gpu_command_encoder = super::gpu::create_gpu_command_encoder_template(scope, None);
    templates.insert("GPUCommandEncoder", tmpl_gpu_command_encoder);
    let tmpl_gpu_compilation_info = super::gpu::create_gpu_compilation_info_template(scope, None);
    templates.insert("GPUCompilationInfo", tmpl_gpu_compilation_info);
    let tmpl_gpu_compilation_message = super::gpu::create_gpu_compilation_message_template(scope, None);
    templates.insert("GPUCompilationMessage", tmpl_gpu_compilation_message);
    let tmpl_gpu_compute_pass_encoder = super::gpu::create_gpu_compute_pass_encoder_template(scope, None);
    templates.insert("GPUComputePassEncoder", tmpl_gpu_compute_pass_encoder);
    let tmpl_gpu_compute_pipeline = super::gpu::create_gpu_compute_pipeline_template(scope, None);
    templates.insert("GPUComputePipeline", tmpl_gpu_compute_pipeline);
    let tmpl_gpu_device_lost_info = super::gpu::create_gpu_device_lost_info_template(scope, None);
    templates.insert("GPUDeviceLostInfo", tmpl_gpu_device_lost_info);
    let tmpl_gpu_error = super::gpu::create_gpu_error_template(scope, None);
    templates.insert("GPUError", tmpl_gpu_error);
    let tmpl_gpu_external_texture = super::gpu::create_gpu_external_texture_template(scope, None);
    templates.insert("GPUExternalTexture", tmpl_gpu_external_texture);
    let tmpl_gpu_pipeline_layout = super::gpu::create_gpu_pipeline_layout_template(scope, None);
    templates.insert("GPUPipelineLayout", tmpl_gpu_pipeline_layout);
    let tmpl_gpu_query_set = super::gpu::create_gpu_query_set_template(scope, None);
    templates.insert("GPUQuerySet", tmpl_gpu_query_set);
    let tmpl_gpu_queue = super::gpu::create_gpu_queue_template(scope, None);
    templates.insert("GPUQueue", tmpl_gpu_queue);
    let tmpl_gpu_render_bundle = super::gpu::create_gpu_render_bundle_template(scope, None);
    templates.insert("GPURenderBundle", tmpl_gpu_render_bundle);
    let tmpl_gpu_render_bundle_encoder = super::gpu::create_gpu_render_bundle_encoder_template(scope, None);
    templates.insert("GPURenderBundleEncoder", tmpl_gpu_render_bundle_encoder);
    let tmpl_gpu_render_pass_encoder = super::gpu::create_gpu_render_pass_encoder_template(scope, None);
    templates.insert("GPURenderPassEncoder", tmpl_gpu_render_pass_encoder);
    let tmpl_gpu_render_pipeline = super::gpu::create_gpu_render_pipeline_template(scope, None);
    templates.insert("GPURenderPipeline", tmpl_gpu_render_pipeline);
    let tmpl_gpu_sampler = super::gpu::create_gpu_sampler_template(scope, None);
    templates.insert("GPUSampler", tmpl_gpu_sampler);
    let tmpl_gpu_shader_module = super::gpu::create_gpu_shader_module_template(scope, None);
    templates.insert("GPUShaderModule", tmpl_gpu_shader_module);
    let tmpl_gpu_supported_features = super::gpu::create_gpu_supported_features_template(scope, None);
    templates.insert("GPUSupportedFeatures", tmpl_gpu_supported_features);
    let tmpl_gpu_supported_limits = super::gpu::create_gpu_supported_limits_template(scope, None);
    templates.insert("GPUSupportedLimits", tmpl_gpu_supported_limits);
    let tmpl_gpu_texture = super::gpu::create_gpu_texture_template(scope, None);
    templates.insert("GPUTexture", tmpl_gpu_texture);
    let tmpl_gpu_texture_view = super::gpu::create_gpu_texture_view_template(scope, None);
    templates.insert("GPUTextureView", tmpl_gpu_texture_view);
    let tmpl_gamepad = super::gamepad::create_gamepad_template(scope, None);
    templates.insert("Gamepad", tmpl_gamepad);
    let tmpl_gamepad_button = super::gamepad::create_gamepad_button_template(scope, None);
    templates.insert("GamepadButton", tmpl_gamepad_button);
    let tmpl_gamepad_haptic_actuator = super::gamepad::create_gamepad_haptic_actuator_template(scope, None);
    templates.insert("GamepadHapticActuator", tmpl_gamepad_haptic_actuator);
    let tmpl_gamepad_pose = super::gamepad::create_gamepad_pose_template(scope, None);
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
    let tmpl_html_all_collection = super::html_elements::create_html_all_collection_template(scope, None);
    templates.insert("HTMLAllCollection", tmpl_html_all_collection);
    let tmpl_html_collection = super::html_elements::create_html_collection_template(scope, None);
    templates.insert("HTMLCollection", tmpl_html_collection);
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
    let tmpl_idb_cursor = super::idb::create_idb_cursor_template(scope, None);
    templates.insert("IDBCursor", tmpl_idb_cursor);
    let tmpl_idb_factory = super::idb::create_idb_factory_template(scope, None);
    templates.insert("IDBFactory", tmpl_idb_factory);
    let tmpl_idb_index = super::idb::create_idb_index_template(scope, None);
    templates.insert("IDBIndex", tmpl_idb_index);
    let tmpl_idb_key_range = super::idb::create_idb_key_range_template(scope, None);
    templates.insert("IDBKeyRange", tmpl_idb_key_range);
    let tmpl_idb_object_store = super::idb::create_idb_object_store_template(scope, None);
    templates.insert("IDBObjectStore", tmpl_idb_object_store);
    let tmpl_idb_record = super::idb::create_idb_record_template(scope, None);
    templates.insert("IDBRecord", tmpl_idb_record);
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
    let tmpl_midi_input_map = super::midi::create_midi_input_map_template(scope, None);
    templates.insert("MIDIInputMap", tmpl_midi_input_map);
    let tmpl_midi_output_map = super::midi::create_midi_output_map_template(scope, None);
    templates.insert("MIDIOutputMap", tmpl_midi_output_map);
    let tmpl_ml = super::web_apis::create_ml_template(scope, None);
    templates.insert("ML", tmpl_ml);
    let tmpl_ml_context = super::web_apis::create_ml_context_template(scope, None);
    templates.insert("MLContext", tmpl_ml_context);
    let tmpl_ml_graph = super::web_apis::create_ml_graph_template(scope, None);
    templates.insert("MLGraph", tmpl_ml_graph);
    let tmpl_ml_graph_builder = super::web_apis::create_ml_graph_builder_template(scope, None);
    templates.insert("MLGraphBuilder", tmpl_ml_graph_builder);
    let tmpl_ml_operand = super::web_apis::create_ml_operand_template(scope, None);
    templates.insert("MLOperand", tmpl_ml_operand);
    let tmpl_ml_tensor = super::web_apis::create_ml_tensor_template(scope, None);
    templates.insert("MLTensor", tmpl_ml_tensor);
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
    let tmpl_ndef_message = super::web_apis::create_ndef_message_template(scope, None);
    templates.insert("NDEFMessage", tmpl_ndef_message);
    let tmpl_ndef_record = super::web_apis::create_ndef_record_template(scope, None);
    templates.insert("NDEFRecord", tmpl_ndef_record);
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
    let tmpl_navigator_ua_data = super::web_apis::create_navigator_ua_data_template(scope, None);
    templates.insert("NavigatorUAData", tmpl_navigator_ua_data);
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
    let tmpl_offscreen_canvas_rendering_context2d = super::web_apis::create_offscreen_canvas_rendering_context2d_template(scope, None);
    templates.insert("OffscreenCanvasRenderingContext2D", tmpl_offscreen_canvas_rendering_context2d);
    let tmpl_origin = super::web_apis::create_origin_template(scope, None);
    templates.insert("Origin", tmpl_origin);
    let tmpl_paint_rendering_context2d = super::web_apis::create_paint_rendering_context2d_template(scope, None);
    templates.insert("PaintRenderingContext2D", tmpl_paint_rendering_context2d);
    let tmpl_paint_size = super::web_apis::create_paint_size_template(scope, None);
    templates.insert("PaintSize", tmpl_paint_size);
    let tmpl_path2d = super::web_apis::create_path2d_template(scope, None);
    templates.insert("Path2D", tmpl_path2d);
    let tmpl_payment_manager = super::payment::create_payment_manager_template(scope, None);
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
    let tmpl_presentation = super::presentation::create_presentation_template(scope, None);
    templates.insert("Presentation", tmpl_presentation);
    let tmpl_presentation_receiver = super::presentation::create_presentation_receiver_template(scope, None);
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
    let tmpl_rtc_certificate = super::webrtc::create_rtc_certificate_template(scope, None);
    templates.insert("RTCCertificate", tmpl_rtc_certificate);
    let tmpl_rtc_encoded_audio_frame = super::webrtc::create_rtc_encoded_audio_frame_template(scope, None);
    templates.insert("RTCEncodedAudioFrame", tmpl_rtc_encoded_audio_frame);
    let tmpl_rtc_encoded_video_frame = super::webrtc::create_rtc_encoded_video_frame_template(scope, None);
    templates.insert("RTCEncodedVideoFrame", tmpl_rtc_encoded_video_frame);
    let tmpl_rtc_ice_candidate = super::webrtc::create_rtc_ice_candidate_template(scope, None);
    templates.insert("RTCIceCandidate", tmpl_rtc_ice_candidate);
    let tmpl_rtc_ice_candidate_pair = super::webrtc::create_rtc_ice_candidate_pair_template(scope, None);
    templates.insert("RTCIceCandidatePair", tmpl_rtc_ice_candidate_pair);
    let tmpl_rtc_identity_assertion = super::webrtc::create_rtc_identity_assertion_template(scope, None);
    templates.insert("RTCIdentityAssertion", tmpl_rtc_identity_assertion);
    let tmpl_rtc_identity_provider_registrar = super::webrtc::create_rtc_identity_provider_registrar_template(scope, None);
    templates.insert("RTCIdentityProviderRegistrar", tmpl_rtc_identity_provider_registrar);
    let tmpl_rtc_rtp_receiver = super::webrtc::create_rtc_rtp_receiver_template(scope, None);
    templates.insert("RTCRtpReceiver", tmpl_rtc_rtp_receiver);
    let tmpl_rtc_rtp_s_frame_encrypter = super::webrtc::create_rtc_rtp_s_frame_encrypter_template(scope, None);
    templates.insert("RTCRtpSFrameEncrypter", tmpl_rtc_rtp_s_frame_encrypter);
    let tmpl_rtc_rtp_script_transform = super::webrtc::create_rtc_rtp_script_transform_template(scope, None);
    templates.insert("RTCRtpScriptTransform", tmpl_rtc_rtp_script_transform);
    let tmpl_rtc_rtp_sender = super::webrtc::create_rtc_rtp_sender_template(scope, None);
    templates.insert("RTCRtpSender", tmpl_rtc_rtp_sender);
    let tmpl_rtc_rtp_transceiver = super::webrtc::create_rtc_rtp_transceiver_template(scope, None);
    templates.insert("RTCRtpTransceiver", tmpl_rtc_rtp_transceiver);
    let tmpl_rtc_session_description = super::webrtc::create_rtc_session_description_template(scope, None);
    templates.insert("RTCSessionDescription", tmpl_rtc_session_description);
    let tmpl_rtc_stats_report = super::webrtc::create_rtc_stats_report_template(scope, None);
    templates.insert("RTCStatsReport", tmpl_rtc_stats_report);
    let tmpl_readable_byte_stream_controller = super::streams::create_readable_byte_stream_controller_template(scope, None);
    templates.insert("ReadableByteStreamController", tmpl_readable_byte_stream_controller);
    let tmpl_readable_stream = super::streams::create_readable_stream_template(scope, None);
    templates.insert("ReadableStream", tmpl_readable_stream);
    let tmpl_readable_stream_byob_reader = super::streams::create_readable_stream_byob_reader_template(scope, None);
    templates.insert("ReadableStreamBYOBReader", tmpl_readable_stream_byob_reader);
    let tmpl_readable_stream_byob_request = super::streams::create_readable_stream_byob_request_template(scope, None);
    templates.insert("ReadableStreamBYOBRequest", tmpl_readable_stream_byob_request);
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
    let tmpl_s_frame_encrypter_stream = super::web_apis::create_s_frame_encrypter_stream_template(scope, None);
    templates.insert("SFrameEncrypterStream", tmpl_s_frame_encrypter_stream);
    let tmpl_svg_angle = super::svg::create_svg_angle_template(scope, None);
    templates.insert("SVGAngle", tmpl_svg_angle);
    let tmpl_svg_animated_angle = super::svg::create_svg_animated_angle_template(scope, None);
    templates.insert("SVGAnimatedAngle", tmpl_svg_animated_angle);
    let tmpl_svg_animated_boolean = super::svg::create_svg_animated_boolean_template(scope, None);
    templates.insert("SVGAnimatedBoolean", tmpl_svg_animated_boolean);
    let tmpl_svg_animated_enumeration = super::svg::create_svg_animated_enumeration_template(scope, None);
    templates.insert("SVGAnimatedEnumeration", tmpl_svg_animated_enumeration);
    let tmpl_svg_animated_integer = super::svg::create_svg_animated_integer_template(scope, None);
    templates.insert("SVGAnimatedInteger", tmpl_svg_animated_integer);
    let tmpl_svg_animated_length = super::svg::create_svg_animated_length_template(scope, None);
    templates.insert("SVGAnimatedLength", tmpl_svg_animated_length);
    let tmpl_svg_animated_length_list = super::svg::create_svg_animated_length_list_template(scope, None);
    templates.insert("SVGAnimatedLengthList", tmpl_svg_animated_length_list);
    let tmpl_svg_animated_number = super::svg::create_svg_animated_number_template(scope, None);
    templates.insert("SVGAnimatedNumber", tmpl_svg_animated_number);
    let tmpl_svg_animated_number_list = super::svg::create_svg_animated_number_list_template(scope, None);
    templates.insert("SVGAnimatedNumberList", tmpl_svg_animated_number_list);
    let tmpl_svg_animated_preserve_aspect_ratio = super::svg::create_svg_animated_preserve_aspect_ratio_template(scope, None);
    templates.insert("SVGAnimatedPreserveAspectRatio", tmpl_svg_animated_preserve_aspect_ratio);
    let tmpl_svg_animated_rect = super::svg::create_svg_animated_rect_template(scope, None);
    templates.insert("SVGAnimatedRect", tmpl_svg_animated_rect);
    let tmpl_svg_animated_string = super::svg::create_svg_animated_string_template(scope, None);
    templates.insert("SVGAnimatedString", tmpl_svg_animated_string);
    let tmpl_svg_animated_transform_list = super::svg::create_svg_animated_transform_list_template(scope, None);
    templates.insert("SVGAnimatedTransformList", tmpl_svg_animated_transform_list);
    let tmpl_svg_length = super::svg::create_svg_length_template(scope, None);
    templates.insert("SVGLength", tmpl_svg_length);
    let tmpl_svg_length_list = super::svg::create_svg_length_list_template(scope, None);
    templates.insert("SVGLengthList", tmpl_svg_length_list);
    let tmpl_svg_number = super::svg::create_svg_number_template(scope, None);
    templates.insert("SVGNumber", tmpl_svg_number);
    let tmpl_svg_number_list = super::svg::create_svg_number_list_template(scope, None);
    templates.insert("SVGNumberList", tmpl_svg_number_list);
    let tmpl_svg_path_segment = super::svg::create_svg_path_segment_template(scope, None);
    templates.insert("SVGPathSegment", tmpl_svg_path_segment);
    let tmpl_svg_point_list = super::svg::create_svg_point_list_template(scope, None);
    templates.insert("SVGPointList", tmpl_svg_point_list);
    let tmpl_svg_preserve_aspect_ratio = super::svg::create_svg_preserve_aspect_ratio_template(scope, None);
    templates.insert("SVGPreserveAspectRatio", tmpl_svg_preserve_aspect_ratio);
    let tmpl_svg_string_list = super::svg::create_svg_string_list_template(scope, None);
    templates.insert("SVGStringList", tmpl_svg_string_list);
    let tmpl_svg_transform = super::svg::create_svg_transform_template(scope, None);
    templates.insert("SVGTransform", tmpl_svg_transform);
    let tmpl_svg_transform_list = super::svg::create_svg_transform_list_template(scope, None);
    templates.insert("SVGTransformList", tmpl_svg_transform_list);
    let tmpl_svg_unit_types = super::svg::create_svg_unit_types_template(scope, None);
    templates.insert("SVGUnitTypes", tmpl_svg_unit_types);
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
    let tmpl_text_decoder = super::encoding::create_text_decoder_template(scope, None);
    templates.insert("TextDecoder", tmpl_text_decoder);
    let tmpl_text_decoder_stream = super::encoding::create_text_decoder_stream_template(scope, None);
    templates.insert("TextDecoderStream", tmpl_text_decoder_stream);
    let tmpl_text_detector = super::web_apis::create_text_detector_template(scope, None);
    templates.insert("TextDetector", tmpl_text_detector);
    let tmpl_text_encoder = super::encoding::create_text_encoder_template(scope, None);
    templates.insert("TextEncoder", tmpl_text_encoder);
    let tmpl_text_encoder_stream = super::encoding::create_text_encoder_stream_template(scope, None);
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
    let tmpl_url = super::url::create_url_template(scope, None);
    templates.insert("URL", tmpl_url);
    let tmpl_url_pattern = super::web_apis::create_url_pattern_template(scope, None);
    templates.insert("URLPattern", tmpl_url_pattern);
    let tmpl_url_search_params = super::url::create_url_search_params_template(scope, None);
    templates.insert("URLSearchParams", tmpl_url_search_params);
    let tmpl_usb_alternate_interface = super::usb::create_usb_alternate_interface_template(scope, None);
    templates.insert("USBAlternateInterface", tmpl_usb_alternate_interface);
    let tmpl_usb_configuration = super::usb::create_usb_configuration_template(scope, None);
    templates.insert("USBConfiguration", tmpl_usb_configuration);
    let tmpl_usb_device = super::usb::create_usb_device_template(scope, None);
    templates.insert("USBDevice", tmpl_usb_device);
    let tmpl_usb_endpoint = super::usb::create_usb_endpoint_template(scope, None);
    templates.insert("USBEndpoint", tmpl_usb_endpoint);
    let tmpl_usb_in_transfer_result = super::usb::create_usb_in_transfer_result_template(scope, None);
    templates.insert("USBInTransferResult", tmpl_usb_in_transfer_result);
    let tmpl_usb_interface = super::usb::create_usb_interface_template(scope, None);
    templates.insert("USBInterface", tmpl_usb_interface);
    let tmpl_usb_isochronous_in_transfer_packet = super::usb::create_usb_isochronous_in_transfer_packet_template(scope, None);
    templates.insert("USBIsochronousInTransferPacket", tmpl_usb_isochronous_in_transfer_packet);
    let tmpl_usb_isochronous_in_transfer_result = super::usb::create_usb_isochronous_in_transfer_result_template(scope, None);
    templates.insert("USBIsochronousInTransferResult", tmpl_usb_isochronous_in_transfer_result);
    let tmpl_usb_isochronous_out_transfer_packet = super::usb::create_usb_isochronous_out_transfer_packet_template(scope, None);
    templates.insert("USBIsochronousOutTransferPacket", tmpl_usb_isochronous_out_transfer_packet);
    let tmpl_usb_isochronous_out_transfer_result = super::usb::create_usb_isochronous_out_transfer_result_template(scope, None);
    templates.insert("USBIsochronousOutTransferResult", tmpl_usb_isochronous_out_transfer_result);
    let tmpl_usb_out_transfer_result = super::usb::create_usb_out_transfer_result_template(scope, None);
    templates.insert("USBOutTransferResult", tmpl_usb_out_transfer_result);
    let tmpl_user_activation = super::web_apis::create_user_activation_template(scope, None);
    templates.insert("UserActivation", tmpl_user_activation);
    let tmpl_vtt_region = super::web_apis::create_vtt_region_template(scope, None);
    templates.insert("VTTRegion", tmpl_vtt_region);
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
    let tmpl_wgsl_language_features = super::web_apis::create_wgsl_language_features_template(scope, None);
    templates.insert("WGSLLanguageFeatures", tmpl_wgsl_language_features);
    let tmpl_wake_lock = super::web_apis::create_wake_lock_template(scope, None);
    templates.insert("WakeLock", tmpl_wake_lock);
    let tmpl_web_gl2rendering_context = super::webgl::create_web_gl2rendering_context_template(scope, None);
    templates.insert("WebGL2RenderingContext", tmpl_web_gl2rendering_context);
    let tmpl_web_gl_active_info = super::webgl::create_web_gl_active_info_template(scope, None);
    templates.insert("WebGLActiveInfo", tmpl_web_gl_active_info);
    let tmpl_web_gl_object = super::webgl::create_web_gl_object_template(scope, None);
    templates.insert("WebGLObject", tmpl_web_gl_object);
    let tmpl_web_gl_rendering_context = super::webgl::create_web_gl_rendering_context_template(scope, None);
    templates.insert("WebGLRenderingContext", tmpl_web_gl_rendering_context);
    let tmpl_web_gl_shader_precision_format = super::webgl::create_web_gl_shader_precision_format_template(scope, None);
    templates.insert("WebGLShaderPrecisionFormat", tmpl_web_gl_shader_precision_format);
    let tmpl_web_gl_uniform_location = super::webgl::create_web_gl_uniform_location_template(scope, None);
    templates.insert("WebGLUniformLocation", tmpl_web_gl_uniform_location);
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
    let tmpl_xml_serializer = super::web_apis::create_xml_serializer_template(scope, None);
    templates.insert("XMLSerializer", tmpl_xml_serializer);
    let tmpl_x_path_evaluator = super::web_apis::create_x_path_evaluator_template(scope, None);
    templates.insert("XPathEvaluator", tmpl_x_path_evaluator);
    let tmpl_x_path_expression = super::web_apis::create_x_path_expression_template(scope, None);
    templates.insert("XPathExpression", tmpl_x_path_expression);
    let tmpl_x_path_result = super::web_apis::create_x_path_result_template(scope, None);
    templates.insert("XPathResult", tmpl_x_path_result);
    let tmpl_xr_anchor = super::webxr::create_xr_anchor_template(scope, None);
    templates.insert("XRAnchor", tmpl_xr_anchor);
    let tmpl_xr_anchor_set = super::webxr::create_xr_anchor_set_template(scope, None);
    templates.insert("XRAnchorSet", tmpl_xr_anchor_set);
    let tmpl_xr_body = super::webxr::create_xr_body_template(scope, None);
    templates.insert("XRBody", tmpl_xr_body);
    let tmpl_xr_camera = super::webxr::create_xr_camera_template(scope, None);
    templates.insert("XRCamera", tmpl_xr_camera);
    let tmpl_xr_depth_information = super::webxr::create_xr_depth_information_template(scope, None);
    templates.insert("XRDepthInformation", tmpl_xr_depth_information);
    let tmpl_xr_frame = super::webxr::create_xr_frame_template(scope, None);
    templates.insert("XRFrame", tmpl_xr_frame);
    let tmpl_xr_hand = super::webxr::create_xr_hand_template(scope, None);
    templates.insert("XRHand", tmpl_xr_hand);
    let tmpl_xr_hit_test_result = super::webxr::create_xr_hit_test_result_template(scope, None);
    templates.insert("XRHitTestResult", tmpl_xr_hit_test_result);
    let tmpl_xr_hit_test_source = super::webxr::create_xr_hit_test_source_template(scope, None);
    templates.insert("XRHitTestSource", tmpl_xr_hit_test_source);
    let tmpl_xr_input_source = super::webxr::create_xr_input_source_template(scope, None);
    templates.insert("XRInputSource", tmpl_xr_input_source);
    let tmpl_xr_input_source_array = super::webxr::create_xr_input_source_array_template(scope, None);
    templates.insert("XRInputSourceArray", tmpl_xr_input_source_array);
    let tmpl_xr_light_estimate = super::webxr::create_xr_light_estimate_template(scope, None);
    templates.insert("XRLightEstimate", tmpl_xr_light_estimate);
    let tmpl_xr_media_binding = super::webxr::create_xr_media_binding_template(scope, None);
    templates.insert("XRMediaBinding", tmpl_xr_media_binding);
    let tmpl_xr_mesh = super::webxr::create_xr_mesh_template(scope, None);
    templates.insert("XRMesh", tmpl_xr_mesh);
    let tmpl_xr_mesh_set = super::webxr::create_xr_mesh_set_template(scope, None);
    templates.insert("XRMeshSet", tmpl_xr_mesh_set);
    let tmpl_xr_plane = super::webxr::create_xr_plane_template(scope, None);
    templates.insert("XRPlane", tmpl_xr_plane);
    let tmpl_xr_plane_set = super::webxr::create_xr_plane_set_template(scope, None);
    templates.insert("XRPlaneSet", tmpl_xr_plane_set);
    let tmpl_xr_pose = super::webxr::create_xr_pose_template(scope, None);
    templates.insert("XRPose", tmpl_xr_pose);
    let tmpl_xr_ray = super::webxr::create_xr_ray_template(scope, None);
    templates.insert("XRRay", tmpl_xr_ray);
    let tmpl_xr_render_state = super::webxr::create_xr_render_state_template(scope, None);
    templates.insert("XRRenderState", tmpl_xr_render_state);
    let tmpl_xr_rigid_transform = super::webxr::create_xr_rigid_transform_template(scope, None);
    templates.insert("XRRigidTransform", tmpl_xr_rigid_transform);
    let tmpl_xr_sub_image = super::webxr::create_xr_sub_image_template(scope, None);
    templates.insert("XRSubImage", tmpl_xr_sub_image);
    let tmpl_xr_transient_input_hit_test_result = super::webxr::create_xr_transient_input_hit_test_result_template(scope, None);
    templates.insert("XRTransientInputHitTestResult", tmpl_xr_transient_input_hit_test_result);
    let tmpl_xr_transient_input_hit_test_source = super::webxr::create_xr_transient_input_hit_test_source_template(scope, None);
    templates.insert("XRTransientInputHitTestSource", tmpl_xr_transient_input_hit_test_source);
    let tmpl_xr_view = super::webxr::create_xr_view_template(scope, None);
    templates.insert("XRView", tmpl_xr_view);
    let tmpl_xr_viewport = super::webxr::create_xr_viewport_template(scope, None);
    templates.insert("XRViewport", tmpl_xr_viewport);
    let tmpl_xr_web_gl_binding = super::webxr::create_xr_web_gl_binding_template(scope, None);
    templates.insert("XRWebGLBinding", tmpl_xr_web_gl_binding);
    let tmpl_xslt_processor = super::web_apis::create_xslt_processor_template(scope, None);
    templates.insert("XSLTProcessor", tmpl_xslt_processor);
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
    let tmpl_webkit_idb_key_range = super::chrome_extensions::create_webkit_idb_key_range_template(scope, None);
    templates.insert("webkitIDBKeyRange", tmpl_webkit_idb_key_range);
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
    let tmpl_webkit_rtc_peer_connection = super::chrome_extensions::create_webkit_rtc_peer_connection_template(scope, None);
    templates.insert("webkitRTCPeerConnection", tmpl_webkit_rtc_peer_connection);
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
    let tmpl_authenticator_assertion_response = super::credentials::create_authenticator_assertion_response_template(scope, templates.get("AuthenticatorResponse").copied());
    templates.insert("AuthenticatorAssertionResponse", tmpl_authenticator_assertion_response);
    let tmpl_authenticator_attestation_response = super::credentials::create_authenticator_attestation_response_template(scope, templates.get("AuthenticatorResponse").copied());
    templates.insert("AuthenticatorAttestationResponse", tmpl_authenticator_attestation_response);
    let tmpl_file = super::web_apis::create_file_template(scope, templates.get("Blob").copied());
    templates.insert("File", tmpl_file);
    let tmpl_css_parser_at_rule = super::css_om::create_css_parser_at_rule_template(scope, templates.get("CSSParserRule").copied());
    templates.insert("CSSParserAtRule", tmpl_css_parser_at_rule);
    let tmpl_css_parser_declaration = super::css_om::create_css_parser_declaration_template(scope, templates.get("CSSParserRule").copied());
    templates.insert("CSSParserDeclaration", tmpl_css_parser_declaration);
    let tmpl_css_parser_qualified_rule = super::css_om::create_css_parser_qualified_rule_template(scope, templates.get("CSSParserRule").copied());
    templates.insert("CSSParserQualifiedRule", tmpl_css_parser_qualified_rule);
    let tmpl_css_parser_block = super::css_om::create_css_parser_block_template(scope, templates.get("CSSParserValue").copied());
    templates.insert("CSSParserBlock", tmpl_css_parser_block);
    let tmpl_css_parser_function = super::css_om::create_css_parser_function_template(scope, templates.get("CSSParserValue").copied());
    templates.insert("CSSParserFunction", tmpl_css_parser_function);
    let tmpl_css_apply_statement_rule = super::css_om::create_css_apply_statement_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSApplyStatementRule", tmpl_css_apply_statement_rule);
    let tmpl_css_color_profile_rule = super::css_om::create_css_color_profile_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSColorProfileRule", tmpl_css_color_profile_rule);
    let tmpl_css_contents_statement_rule = super::css_om::create_css_contents_statement_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSContentsStatementRule", tmpl_css_contents_statement_rule);
    let tmpl_css_counter_style_rule = super::css_om::create_css_counter_style_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSCounterStyleRule", tmpl_css_counter_style_rule);
    let tmpl_css_custom_media_rule = super::css_om::create_css_custom_media_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSCustomMediaRule", tmpl_css_custom_media_rule);
    let tmpl_css_font_face_rule = super::css_om::create_css_font_face_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSFontFaceRule", tmpl_css_font_face_rule);
    let tmpl_css_font_feature_values_rule = super::css_om::create_css_font_feature_values_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSFontFeatureValuesRule", tmpl_css_font_feature_values_rule);
    let tmpl_css_font_palette_values_rule = super::css_om::create_css_font_palette_values_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSFontPaletteValuesRule", tmpl_css_font_palette_values_rule);
    let tmpl_css_function_declarations = super::css_om::create_css_function_declarations_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSFunctionDeclarations", tmpl_css_function_declarations);
    let tmpl_css_grouping_rule = super::css_om::create_css_grouping_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSGroupingRule", tmpl_css_grouping_rule);
    let tmpl_css_import_rule = super::css_om::create_css_import_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSImportRule", tmpl_css_import_rule);
    let tmpl_css_keyframe_rule = super::css_om::create_css_keyframe_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSKeyframeRule", tmpl_css_keyframe_rule);
    let tmpl_css_keyframes_rule = super::css_om::create_css_keyframes_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSKeyframesRule", tmpl_css_keyframes_rule);
    let tmpl_css_layer_statement_rule = super::css_om::create_css_layer_statement_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSLayerStatementRule", tmpl_css_layer_statement_rule);
    let tmpl_css_margin_rule = super::css_om::create_css_margin_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSMarginRule", tmpl_css_margin_rule);
    let tmpl_css_namespace_rule = super::css_om::create_css_namespace_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSNamespaceRule", tmpl_css_namespace_rule);
    let tmpl_css_nested_declarations = super::css_om::create_css_nested_declarations_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSNestedDeclarations", tmpl_css_nested_declarations);
    let tmpl_css_position_try_rule = super::css_om::create_css_position_try_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSPositionTryRule", tmpl_css_position_try_rule);
    let tmpl_css_property_rule = super::css_om::create_css_property_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSPropertyRule", tmpl_css_property_rule);
    let tmpl_css_view_transition_rule = super::css_om::create_css_view_transition_rule_template(scope, templates.get("CSSRule").copied());
    templates.insert("CSSViewTransitionRule", tmpl_css_view_transition_rule);
    let tmpl_css_font_face_descriptors = super::css_om::create_css_font_face_descriptors_template(scope, templates.get("CSSStyleDeclaration").copied());
    templates.insert("CSSFontFaceDescriptors", tmpl_css_font_face_descriptors);
    let tmpl_css_function_descriptors = super::css_om::create_css_function_descriptors_template(scope, templates.get("CSSStyleDeclaration").copied());
    templates.insert("CSSFunctionDescriptors", tmpl_css_function_descriptors);
    let tmpl_css_page_descriptors = super::css_om::create_css_page_descriptors_template(scope, templates.get("CSSStyleDeclaration").copied());
    templates.insert("CSSPageDescriptors", tmpl_css_page_descriptors);
    let tmpl_css_position_try_descriptors = super::css_om::create_css_position_try_descriptors_template(scope, templates.get("CSSStyleDeclaration").copied());
    templates.insert("CSSPositionTryDescriptors", tmpl_css_position_try_descriptors);
    let tmpl_css_style_properties = super::css_om::create_css_style_properties_template(scope, templates.get("CSSStyleDeclaration").copied());
    templates.insert("CSSStyleProperties", tmpl_css_style_properties);
    let tmpl_css_color_value = super::css_om::create_css_color_value_template(scope, templates.get("CSSStyleValue").copied());
    templates.insert("CSSColorValue", tmpl_css_color_value);
    let tmpl_css_image_value = super::css_om::create_css_image_value_template(scope, templates.get("CSSStyleValue").copied());
    templates.insert("CSSImageValue", tmpl_css_image_value);
    let tmpl_css_keyword_value = super::css_om::create_css_keyword_value_template(scope, templates.get("CSSStyleValue").copied());
    templates.insert("CSSKeywordValue", tmpl_css_keyword_value);
    let tmpl_css_numeric_value = super::css_om::create_css_numeric_value_template(scope, templates.get("CSSStyleValue").copied());
    templates.insert("CSSNumericValue", tmpl_css_numeric_value);
    let tmpl_css_transform_value = super::css_om::create_css_transform_value_template(scope, templates.get("CSSStyleValue").copied());
    templates.insert("CSSTransformValue", tmpl_css_transform_value);
    let tmpl_css_unparsed_value = super::css_om::create_css_unparsed_value_template(scope, templates.get("CSSStyleValue").copied());
    templates.insert("CSSUnparsedValue", tmpl_css_unparsed_value);
    let tmpl_css_matrix_component = super::css_om::create_css_matrix_component_template(scope, templates.get("CSSTransformComponent").copied());
    templates.insert("CSSMatrixComponent", tmpl_css_matrix_component);
    let tmpl_css_perspective = super::css_om::create_css_perspective_template(scope, templates.get("CSSTransformComponent").copied());
    templates.insert("CSSPerspective", tmpl_css_perspective);
    let tmpl_css_rotate = super::css_om::create_css_rotate_template(scope, templates.get("CSSTransformComponent").copied());
    templates.insert("CSSRotate", tmpl_css_rotate);
    let tmpl_css_scale = super::css_om::create_css_scale_template(scope, templates.get("CSSTransformComponent").copied());
    templates.insert("CSSScale", tmpl_css_scale);
    let tmpl_css_skew = super::css_om::create_css_skew_template(scope, templates.get("CSSTransformComponent").copied());
    templates.insert("CSSSkew", tmpl_css_skew);
    let tmpl_css_skew_x = super::css_om::create_css_skew_x_template(scope, templates.get("CSSTransformComponent").copied());
    templates.insert("CSSSkewX", tmpl_css_skew_x);
    let tmpl_css_skew_y = super::css_om::create_css_skew_y_template(scope, templates.get("CSSTransformComponent").copied());
    templates.insert("CSSSkewY", tmpl_css_skew_y);
    let tmpl_css_translate = super::css_om::create_css_translate_template(scope, templates.get("CSSTransformComponent").copied());
    templates.insert("CSSTranslate", tmpl_css_translate);
    let tmpl_window_client = super::web_apis::create_window_client_template(scope, templates.get("Client").copied());
    templates.insert("WindowClient", tmpl_window_client);
    let tmpl_digital_credential = super::web_apis::create_digital_credential_template(scope, templates.get("Credential").copied());
    templates.insert("DigitalCredential", tmpl_digital_credential);
    let tmpl_federated_credential = super::web_apis::create_federated_credential_template(scope, templates.get("Credential").copied());
    templates.insert("FederatedCredential", tmpl_federated_credential);
    let tmpl_identity_credential = super::web_apis::create_identity_credential_template(scope, templates.get("Credential").copied());
    templates.insert("IdentityCredential", tmpl_identity_credential);
    let tmpl_otp_credential = super::web_apis::create_otp_credential_template(scope, templates.get("Credential").copied());
    templates.insert("OTPCredential", tmpl_otp_credential);
    let tmpl_password_credential = super::web_apis::create_password_credential_template(scope, templates.get("Credential").copied());
    templates.insert("PasswordCredential", tmpl_password_credential);
    let tmpl_public_key_credential = super::credentials::create_public_key_credential_template(scope, templates.get("Credential").copied());
    templates.insert("PublicKeyCredential", tmpl_public_key_credential);
    let tmpl_gpu_pipeline_error = super::gpu::create_gpu_pipeline_error_template(scope, templates.get("DOMException").copied());
    templates.insert("GPUPipelineError", tmpl_gpu_pipeline_error);
    let tmpl_identity_credential_error = super::web_apis::create_identity_credential_error_template(scope, templates.get("DOMException").copied());
    templates.insert("IdentityCredentialError", tmpl_identity_credential_error);
    let tmpl_overconstrained_error = super::web_apis::create_overconstrained_error_template(scope, templates.get("DOMException").copied());
    templates.insert("OverconstrainedError", tmpl_overconstrained_error);
    let tmpl_quota_exceeded_error = super::web_apis::create_quota_exceeded_error_template(scope, templates.get("DOMException").copied());
    templates.insert("QuotaExceededError", tmpl_quota_exceeded_error);
    let tmpl_rtc_error = super::webrtc::create_rtc_error_template(scope, templates.get("DOMException").copied());
    templates.insert("RTCError", tmpl_rtc_error);
    let tmpl_web_transport_error = super::web_apis::create_web_transport_error_template(scope, templates.get("DOMException").copied());
    templates.insert("WebTransportError", tmpl_web_transport_error);
    let tmpl_dom_matrix = super::dom_core::create_dom_matrix_template(scope, templates.get("DOMMatrixReadOnly").copied());
    templates.insert("DOMMatrix", tmpl_dom_matrix);
    let tmpl_dom_point = super::dom_core::create_dom_point_template(scope, templates.get("DOMPointReadOnly").copied());
    templates.insert("DOMPoint", tmpl_dom_point);
    let tmpl_dom_rect = super::dom_core::create_dom_rect_template(scope, templates.get("DOMRectReadOnly").copied());
    templates.insert("DOMRect", tmpl_dom_rect);
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
    let tmpl_gpu_uncaptured_error_event = super::events::create_gpu_uncaptured_error_event_template(scope, templates.get("Event").copied());
    templates.insert("GPUUncapturedErrorEvent", tmpl_gpu_uncaptured_error_event);
    let tmpl_gamepad_event = super::events::create_gamepad_event_template(scope, templates.get("Event").copied());
    templates.insert("GamepadEvent", tmpl_gamepad_event);
    let tmpl_hid_connection_event = super::events::create_hid_connection_event_template(scope, templates.get("Event").copied());
    templates.insert("HIDConnectionEvent", tmpl_hid_connection_event);
    let tmpl_hid_input_report_event = super::events::create_hid_input_report_event_template(scope, templates.get("Event").copied());
    templates.insert("HIDInputReportEvent", tmpl_hid_input_report_event);
    let tmpl_hash_change_event = super::events::create_hash_change_event_template(scope, templates.get("Event").copied());
    templates.insert("HashChangeEvent", tmpl_hash_change_event);
    let tmpl_idb_version_change_event = super::events::create_idb_version_change_event_template(scope, templates.get("Event").copied());
    templates.insert("IDBVersionChangeEvent", tmpl_idb_version_change_event);
    let tmpl_key_frame_request_event = super::events::create_key_frame_request_event_template(scope, templates.get("Event").copied());
    templates.insert("KeyFrameRequestEvent", tmpl_key_frame_request_event);
    let tmpl_midi_connection_event = super::events::create_midi_connection_event_template(scope, templates.get("Event").copied());
    templates.insert("MIDIConnectionEvent", tmpl_midi_connection_event);
    let tmpl_midi_message_event = super::events::create_midi_message_event_template(scope, templates.get("Event").copied());
    templates.insert("MIDIMessageEvent", tmpl_midi_message_event);
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
    let tmpl_ndef_reading_event = super::events::create_ndef_reading_event_template(scope, templates.get("Event").copied());
    templates.insert("NDEFReadingEvent", tmpl_ndef_reading_event);
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
    let tmpl_rtcdtmf_tone_change_event = super::events::create_rtcdtmf_tone_change_event_template(scope, templates.get("Event").copied());
    templates.insert("RTCDTMFToneChangeEvent", tmpl_rtcdtmf_tone_change_event);
    let tmpl_rtc_data_channel_event = super::events::create_rtc_data_channel_event_template(scope, templates.get("Event").copied());
    templates.insert("RTCDataChannelEvent", tmpl_rtc_data_channel_event);
    let tmpl_rtc_error_event = super::events::create_rtc_error_event_template(scope, templates.get("Event").copied());
    templates.insert("RTCErrorEvent", tmpl_rtc_error_event);
    let tmpl_rtc_peer_connection_ice_error_event = super::events::create_rtc_peer_connection_ice_error_event_template(scope, templates.get("Event").copied());
    templates.insert("RTCPeerConnectionIceErrorEvent", tmpl_rtc_peer_connection_ice_error_event);
    let tmpl_rtc_peer_connection_ice_event = super::events::create_rtc_peer_connection_ice_event_template(scope, templates.get("Event").copied());
    templates.insert("RTCPeerConnectionIceEvent", tmpl_rtc_peer_connection_ice_event);
    let tmpl_rtc_track_event = super::events::create_rtc_track_event_template(scope, templates.get("Event").copied());
    templates.insert("RTCTrackEvent", tmpl_rtc_track_event);
    let tmpl_rtc_transform_event = super::events::create_rtc_transform_event_template(scope, templates.get("Event").copied());
    templates.insert("RTCTransformEvent", tmpl_rtc_transform_event);
    let tmpl_s_frame_transform_error_event = super::events::create_s_frame_transform_error_event_template(scope, templates.get("Event").copied());
    templates.insert("SFrameTransformErrorEvent", tmpl_s_frame_transform_error_event);
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
    let tmpl_ui_event = super::events::create_ui_event_template(scope, templates.get("Event").copied());
    templates.insert("UIEvent", tmpl_ui_event);
    let tmpl_usb_connection_event = super::events::create_usb_connection_event_template(scope, templates.get("Event").copied());
    templates.insert("USBConnectionEvent", tmpl_usb_connection_event);
    let tmpl_value_event = super::events::create_value_event_template(scope, templates.get("Event").copied());
    templates.insert("ValueEvent", tmpl_value_event);
    let tmpl_web_gl_context_event = super::events::create_web_gl_context_event_template(scope, templates.get("Event").copied());
    templates.insert("WebGLContextEvent", tmpl_web_gl_context_event);
    let tmpl_window_controls_overlay_geometry_change_event = super::events::create_window_controls_overlay_geometry_change_event_template(scope, templates.get("Event").copied());
    templates.insert("WindowControlsOverlayGeometryChangeEvent", tmpl_window_controls_overlay_geometry_change_event);
    let tmpl_xr_input_source_event = super::webxr::create_xr_input_source_event_template(scope, templates.get("Event").copied());
    templates.insert("XRInputSourceEvent", tmpl_xr_input_source_event);
    let tmpl_xr_input_sources_change_event = super::webxr::create_xr_input_sources_change_event_template(scope, templates.get("Event").copied());
    templates.insert("XRInputSourcesChangeEvent", tmpl_xr_input_sources_change_event);
    let tmpl_xr_layer_event = super::webxr::create_xr_layer_event_template(scope, templates.get("Event").copied());
    templates.insert("XRLayerEvent", tmpl_xr_layer_event);
    let tmpl_xr_reference_space_event = super::webxr::create_xr_reference_space_event_template(scope, templates.get("Event").copied());
    templates.insert("XRReferenceSpaceEvent", tmpl_xr_reference_space_event);
    let tmpl_xr_session_event = super::webxr::create_xr_session_event_template(scope, templates.get("Event").copied());
    templates.insert("XRSessionEvent", tmpl_xr_session_event);
    let tmpl_xr_visibility_mask_change_event = super::webxr::create_xr_visibility_mask_change_event_template(scope, templates.get("Event").copied());
    templates.insert("XRVisibilityMaskChangeEvent", tmpl_xr_visibility_mask_change_event);
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
    let tmpl_bluetooth_remote_gatt_characteristic = super::bluetooth::create_bluetooth_remote_gatt_characteristic_template(scope, templates.get("EventTarget").copied());
    templates.insert("BluetoothRemoteGATTCharacteristic", tmpl_bluetooth_remote_gatt_characteristic);
    let tmpl_bluetooth_remote_gatt_service = super::bluetooth::create_bluetooth_remote_gatt_service_template(scope, templates.get("EventTarget").copied());
    templates.insert("BluetoothRemoteGATTService", tmpl_bluetooth_remote_gatt_service);
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
    let tmpl_gpu_device = super::gpu::create_gpu_device_template(scope, templates.get("EventTarget").copied());
    templates.insert("GPUDevice", tmpl_gpu_device);
    let tmpl_hid = super::hid::create_hid_template(scope, templates.get("EventTarget").copied());
    templates.insert("HID", tmpl_hid);
    let tmpl_hid_device = super::hid::create_hid_device_template(scope, templates.get("EventTarget").copied());
    templates.insert("HIDDevice", tmpl_hid_device);
    let tmpl_idb_database = super::idb::create_idb_database_template(scope, templates.get("EventTarget").copied());
    templates.insert("IDBDatabase", tmpl_idb_database);
    let tmpl_idb_request = super::idb::create_idb_request_template(scope, templates.get("EventTarget").copied());
    templates.insert("IDBRequest", tmpl_idb_request);
    let tmpl_idb_transaction = super::idb::create_idb_transaction_template(scope, templates.get("EventTarget").copied());
    templates.insert("IDBTransaction", tmpl_idb_transaction);
    let tmpl_idle_detector = super::web_apis::create_idle_detector_template(scope, templates.get("EventTarget").copied());
    templates.insert("IdleDetector", tmpl_idle_detector);
    let tmpl_keyboard = super::web_apis::create_keyboard_template(scope, templates.get("EventTarget").copied());
    templates.insert("Keyboard", tmpl_keyboard);
    let tmpl_language_model = super::web_apis::create_language_model_template(scope, templates.get("EventTarget").copied());
    templates.insert("LanguageModel", tmpl_language_model);
    let tmpl_midi_access = super::midi::create_midi_access_template(scope, templates.get("EventTarget").copied());
    templates.insert("MIDIAccess", tmpl_midi_access);
    let tmpl_midi_port = super::midi::create_midi_port_template(scope, templates.get("EventTarget").copied());
    templates.insert("MIDIPort", tmpl_midi_port);
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
    let tmpl_ndef_reader = super::web_apis::create_ndef_reader_template(scope, templates.get("EventTarget").copied());
    templates.insert("NDEFReader", tmpl_ndef_reader);
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
    let tmpl_payment_request = super::payment::create_payment_request_template(scope, templates.get("EventTarget").copied());
    templates.insert("PaymentRequest", tmpl_payment_request);
    let tmpl_payment_response = super::payment::create_payment_response_template(scope, templates.get("EventTarget").copied());
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
    let tmpl_presentation_availability = super::presentation::create_presentation_availability_template(scope, templates.get("EventTarget").copied());
    templates.insert("PresentationAvailability", tmpl_presentation_availability);
    let tmpl_presentation_connection = super::presentation::create_presentation_connection_template(scope, templates.get("EventTarget").copied());
    templates.insert("PresentationConnection", tmpl_presentation_connection);
    let tmpl_presentation_connection_list = super::presentation::create_presentation_connection_list_template(scope, templates.get("EventTarget").copied());
    templates.insert("PresentationConnectionList", tmpl_presentation_connection_list);
    let tmpl_presentation_request = super::presentation::create_presentation_request_template(scope, templates.get("EventTarget").copied());
    templates.insert("PresentationRequest", tmpl_presentation_request);
    let tmpl_profiler = super::web_apis::create_profiler_template(scope, templates.get("EventTarget").copied());
    templates.insert("Profiler", tmpl_profiler);
    let tmpl_rtcdtmf_sender = super::webrtc::create_rtcdtmf_sender_template(scope, templates.get("EventTarget").copied());
    templates.insert("RTCDTMFSender", tmpl_rtcdtmf_sender);
    let tmpl_rtc_data_channel = super::webrtc::create_rtc_data_channel_template(scope, templates.get("EventTarget").copied());
    templates.insert("RTCDataChannel", tmpl_rtc_data_channel);
    let tmpl_rtc_dtls_transport = super::webrtc::create_rtc_dtls_transport_template(scope, templates.get("EventTarget").copied());
    templates.insert("RTCDtlsTransport", tmpl_rtc_dtls_transport);
    let tmpl_rtc_ice_transport = super::webrtc::create_rtc_ice_transport_template(scope, templates.get("EventTarget").copied());
    templates.insert("RTCIceTransport", tmpl_rtc_ice_transport);
    let tmpl_rtc_peer_connection = super::webrtc::create_rtc_peer_connection_template(scope, templates.get("EventTarget").copied());
    templates.insert("RTCPeerConnection", tmpl_rtc_peer_connection);
    let tmpl_rtc_rtp_s_frame_decrypter = super::webrtc::create_rtc_rtp_s_frame_decrypter_template(scope, templates.get("EventTarget").copied());
    templates.insert("RTCRtpSFrameDecrypter", tmpl_rtc_rtp_s_frame_decrypter);
    let tmpl_rtc_rtp_script_transformer = super::webrtc::create_rtc_rtp_script_transformer_template(scope, templates.get("EventTarget").copied());
    templates.insert("RTCRtpScriptTransformer", tmpl_rtc_rtp_script_transformer);
    let tmpl_rtc_sctp_transport = super::webrtc::create_rtc_sctp_transport_template(scope, templates.get("EventTarget").copied());
    templates.insert("RTCSctpTransport", tmpl_rtc_sctp_transport);
    let tmpl_remote_playback = super::web_apis::create_remote_playback_template(scope, templates.get("EventTarget").copied());
    templates.insert("RemotePlayback", tmpl_remote_playback);
    let tmpl_s_frame_decrypter_stream = super::web_apis::create_s_frame_decrypter_stream_template(scope, templates.get("EventTarget").copied());
    templates.insert("SFrameDecrypterStream", tmpl_s_frame_decrypter_stream);
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
    let tmpl_usb = super::usb::create_usb_template(scope, templates.get("EventTarget").copied());
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
    let tmpl_xml_http_request_event_target = super::web_apis::create_xml_http_request_event_target_template(scope, templates.get("EventTarget").copied());
    templates.insert("XMLHttpRequestEventTarget", tmpl_xml_http_request_event_target);
    let tmpl_xr_layer = super::webxr::create_xr_layer_template(scope, templates.get("EventTarget").copied());
    templates.insert("XRLayer", tmpl_xr_layer);
    let tmpl_xr_light_probe = super::webxr::create_xr_light_probe_template(scope, templates.get("EventTarget").copied());
    templates.insert("XRLightProbe", tmpl_xr_light_probe);
    let tmpl_xr_session = super::webxr::create_xr_session_template(scope, templates.get("EventTarget").copied());
    templates.insert("XRSession", tmpl_xr_session);
    let tmpl_xr_space = super::webxr::create_xr_space_template(scope, templates.get("EventTarget").copied());
    templates.insert("XRSpace", tmpl_xr_space);
    let tmpl_xr_system = super::webxr::create_xr_system_template(scope, templates.get("EventTarget").copied());
    templates.insert("XRSystem", tmpl_xr_system);
    let tmpl_file_system_directory_entry = super::web_apis::create_file_system_directory_entry_template(scope, templates.get("FileSystemEntry").copied());
    templates.insert("FileSystemDirectoryEntry", tmpl_file_system_directory_entry);
    let tmpl_file_system_file_entry = super::web_apis::create_file_system_file_entry_template(scope, templates.get("FileSystemEntry").copied());
    templates.insert("FileSystemFileEntry", tmpl_file_system_file_entry);
    let tmpl_file_system_directory_handle = super::web_apis::create_file_system_directory_handle_template(scope, templates.get("FileSystemHandle").copied());
    templates.insert("FileSystemDirectoryHandle", tmpl_file_system_directory_handle);
    let tmpl_file_system_file_handle = super::web_apis::create_file_system_file_handle_template(scope, templates.get("FileSystemHandle").copied());
    templates.insert("FileSystemFileHandle", tmpl_file_system_file_handle);
    let tmpl_gpu_internal_error = super::gpu::create_gpu_internal_error_template(scope, templates.get("GPUError").copied());
    templates.insert("GPUInternalError", tmpl_gpu_internal_error);
    let tmpl_gpu_out_of_memory_error = super::gpu::create_gpu_out_of_memory_error_template(scope, templates.get("GPUError").copied());
    templates.insert("GPUOutOfMemoryError", tmpl_gpu_out_of_memory_error);
    let tmpl_gpu_validation_error = super::gpu::create_gpu_validation_error_template(scope, templates.get("GPUError").copied());
    templates.insert("GPUValidationError", tmpl_gpu_validation_error);
    let tmpl_sequence_effect = super::web_apis::create_sequence_effect_template(scope, templates.get("GroupEffect").copied());
    templates.insert("SequenceEffect", tmpl_sequence_effect);
    let tmpl_html_form_controls_collection = super::html_elements::create_html_form_controls_collection_template(scope, templates.get("HTMLCollection").copied());
    templates.insert("HTMLFormControlsCollection", tmpl_html_form_controls_collection);
    let tmpl_html_options_collection = super::html_elements::create_html_options_collection_template(scope, templates.get("HTMLCollection").copied());
    templates.insert("HTMLOptionsCollection", tmpl_html_options_collection);
    let tmpl_idb_cursor_with_value = super::idb::create_idb_cursor_with_value_template(scope, templates.get("IDBCursor").copied());
    templates.insert("IDBCursorWithValue", tmpl_idb_cursor_with_value);
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
    let tmpl_css_style_sheet = super::css_om::create_css_style_sheet_template(scope, templates.get("StyleSheet").copied());
    templates.insert("CSSStyleSheet", tmpl_css_style_sheet);
    let tmpl_web_gl_buffer = super::webgl::create_web_gl_buffer_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLBuffer", tmpl_web_gl_buffer);
    let tmpl_web_gl_framebuffer = super::webgl::create_web_gl_framebuffer_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLFramebuffer", tmpl_web_gl_framebuffer);
    let tmpl_web_gl_program = super::webgl::create_web_gl_program_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLProgram", tmpl_web_gl_program);
    let tmpl_web_gl_query = super::webgl::create_web_gl_query_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLQuery", tmpl_web_gl_query);
    let tmpl_web_gl_renderbuffer = super::webgl::create_web_gl_renderbuffer_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLRenderbuffer", tmpl_web_gl_renderbuffer);
    let tmpl_web_gl_sampler = super::webgl::create_web_gl_sampler_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLSampler", tmpl_web_gl_sampler);
    let tmpl_web_gl_shader = super::webgl::create_web_gl_shader_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLShader", tmpl_web_gl_shader);
    let tmpl_web_gl_sync = super::webgl::create_web_gl_sync_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLSync", tmpl_web_gl_sync);
    let tmpl_web_gl_texture = super::webgl::create_web_gl_texture_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLTexture", tmpl_web_gl_texture);
    let tmpl_web_gl_timer_query_ext = super::webgl::create_web_gl_timer_query_ext_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLTimerQueryEXT", tmpl_web_gl_timer_query_ext);
    let tmpl_web_gl_transform_feedback = super::webgl::create_web_gl_transform_feedback_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLTransformFeedback", tmpl_web_gl_transform_feedback);
    let tmpl_web_gl_vertex_array_object = super::webgl::create_web_gl_vertex_array_object_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLVertexArrayObject", tmpl_web_gl_vertex_array_object);
    let tmpl_web_gl_vertex_array_object_oes = super::webgl::create_web_gl_vertex_array_object_oes_template(scope, templates.get("WebGLObject").copied());
    templates.insert("WebGLVertexArrayObjectOES", tmpl_web_gl_vertex_array_object_oes);
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
    let tmpl_xrcpu_depth_information = super::webxr::create_xrcpu_depth_information_template(scope, templates.get("XRDepthInformation").copied());
    templates.insert("XRCPUDepthInformation", tmpl_xrcpu_depth_information);
    let tmpl_xr_web_gl_depth_information = super::webxr::create_xr_web_gl_depth_information_template(scope, templates.get("XRDepthInformation").copied());
    templates.insert("XRWebGLDepthInformation", tmpl_xr_web_gl_depth_information);
    let tmpl_xr_joint_pose = super::webxr::create_xr_joint_pose_template(scope, templates.get("XRPose").copied());
    templates.insert("XRJointPose", tmpl_xr_joint_pose);
    let tmpl_xr_viewer_pose = super::webxr::create_xr_viewer_pose_template(scope, templates.get("XRPose").copied());
    templates.insert("XRViewerPose", tmpl_xr_viewer_pose);
    let tmpl_xr_web_gl_sub_image = super::webxr::create_xr_web_gl_sub_image_template(scope, templates.get("XRSubImage").copied());
    templates.insert("XRWebGLSubImage", tmpl_xr_web_gl_sub_image);
    let tmpl_view_timeline = super::web_apis::create_view_timeline_template(scope, templates.get("ScrollTimeline").copied());
    templates.insert("ViewTimeline", tmpl_view_timeline);
    let tmpl_css_apply_block_rule = super::css_om::create_css_apply_block_rule_template(scope, templates.get("CSSGroupingRule").copied());
    templates.insert("CSSApplyBlockRule", tmpl_css_apply_block_rule);
    let tmpl_css_condition_rule = super::css_om::create_css_condition_rule_template(scope, templates.get("CSSGroupingRule").copied());
    templates.insert("CSSConditionRule", tmpl_css_condition_rule);
    let tmpl_css_contents_block_rule = super::css_om::create_css_contents_block_rule_template(scope, templates.get("CSSGroupingRule").copied());
    templates.insert("CSSContentsBlockRule", tmpl_css_contents_block_rule);
    let tmpl_css_function_rule = super::css_om::create_css_function_rule_template(scope, templates.get("CSSGroupingRule").copied());
    templates.insert("CSSFunctionRule", tmpl_css_function_rule);
    let tmpl_css_layer_block_rule = super::css_om::create_css_layer_block_rule_template(scope, templates.get("CSSGroupingRule").copied());
    templates.insert("CSSLayerBlockRule", tmpl_css_layer_block_rule);
    let tmpl_css_mixin_rule = super::css_om::create_css_mixin_rule_template(scope, templates.get("CSSGroupingRule").copied());
    templates.insert("CSSMixinRule", tmpl_css_mixin_rule);
    let tmpl_css_page_rule = super::css_om::create_css_page_rule_template(scope, templates.get("CSSGroupingRule").copied());
    templates.insert("CSSPageRule", tmpl_css_page_rule);
    let tmpl_css_scope_rule = super::css_om::create_css_scope_rule_template(scope, templates.get("CSSGroupingRule").copied());
    templates.insert("CSSScopeRule", tmpl_css_scope_rule);
    let tmpl_css_starting_style_rule = super::css_om::create_css_starting_style_rule_template(scope, templates.get("CSSGroupingRule").copied());
    templates.insert("CSSStartingStyleRule", tmpl_css_starting_style_rule);
    let tmpl_css_style_rule = super::css_om::create_css_style_rule_template(scope, templates.get("CSSGroupingRule").copied());
    templates.insert("CSSStyleRule", tmpl_css_style_rule);
    let tmpl_css_supports_condition_rule = super::css_om::create_css_supports_condition_rule_template(scope, templates.get("CSSGroupingRule").copied());
    templates.insert("CSSSupportsConditionRule", tmpl_css_supports_condition_rule);
    let tmpl_css_color = super::css_om::create_css_color_template(scope, templates.get("CSSColorValue").copied());
    templates.insert("CSSColor", tmpl_css_color);
    let tmpl_csshsl = super::css_om::create_csshsl_template(scope, templates.get("CSSColorValue").copied());
    templates.insert("CSSHSL", tmpl_csshsl);
    let tmpl_csshwb = super::css_om::create_csshwb_template(scope, templates.get("CSSColorValue").copied());
    templates.insert("CSSHWB", tmpl_csshwb);
    let tmpl_csslch = super::css_om::create_csslch_template(scope, templates.get("CSSColorValue").copied());
    templates.insert("CSSLCH", tmpl_csslch);
    let tmpl_css_lab = super::css_om::create_css_lab_template(scope, templates.get("CSSColorValue").copied());
    templates.insert("CSSLab", tmpl_css_lab);
    let tmpl_cssoklch = super::css_om::create_cssoklch_template(scope, templates.get("CSSColorValue").copied());
    templates.insert("CSSOKLCH", tmpl_cssoklch);
    let tmpl_cssok_lab = super::css_om::create_cssok_lab_template(scope, templates.get("CSSColorValue").copied());
    templates.insert("CSSOKLab", tmpl_cssok_lab);
    let tmpl_cssrgb = super::css_om::create_cssrgb_template(scope, templates.get("CSSColorValue").copied());
    templates.insert("CSSRGB", tmpl_cssrgb);
    let tmpl_css_math_value = super::css_om::create_css_math_value_template(scope, templates.get("CSSNumericValue").copied());
    templates.insert("CSSMathValue", tmpl_css_math_value);
    let tmpl_css_unit_value = super::css_om::create_css_unit_value_template(scope, templates.get("CSSNumericValue").copied());
    templates.insert("CSSUnitValue", tmpl_css_unit_value);
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
    let tmpl_css_animation = super::css_om::create_css_animation_template(scope, templates.get("Animation").copied());
    templates.insert("CSSAnimation", tmpl_css_animation);
    let tmpl_css_transition = super::css_om::create_css_transition_template(scope, templates.get("Animation").copied());
    templates.insert("CSSTransition", tmpl_css_transition);
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
    let tmpl_iir_filter_node = super::web_apis::create_iir_filter_node_template(scope, templates.get("AudioNode").copied());
    templates.insert("IIRFilterNode", tmpl_iir_filter_node);
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
    let tmpl_idb_open_db_request = super::idb::create_idb_open_db_request_template(scope, templates.get("IDBRequest").copied());
    templates.insert("IDBOpenDBRequest", tmpl_idb_open_db_request);
    let tmpl_midi_input = super::midi::create_midi_input_template(scope, templates.get("MIDIPort").copied());
    templates.insert("MIDIInput", tmpl_midi_input);
    let tmpl_midi_output = super::midi::create_midi_output_template(scope, templates.get("MIDIPort").copied());
    templates.insert("MIDIOutput", tmpl_midi_output);
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
    let tmpl_bluetooth_le_scan_permission_result = super::bluetooth::create_bluetooth_le_scan_permission_result_template(scope, templates.get("PermissionStatus").copied());
    templates.insert("BluetoothLEScanPermissionResult", tmpl_bluetooth_le_scan_permission_result);
    let tmpl_bluetooth_permission_result = super::bluetooth::create_bluetooth_permission_result_template(scope, templates.get("PermissionStatus").copied());
    templates.insert("BluetoothPermissionResult", tmpl_bluetooth_permission_result);
    let tmpl_usb_permission_result = super::usb::create_usb_permission_result_template(scope, templates.get("PermissionStatus").copied());
    templates.insert("USBPermissionResult", tmpl_usb_permission_result);
    let tmpl_xr_permission_status = super::webxr::create_xr_permission_status_template(scope, templates.get("PermissionStatus").copied());
    templates.insert("XRPermissionStatus", tmpl_xr_permission_status);
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
    let tmpl_vtt_cue = super::web_apis::create_vtt_cue_template(scope, templates.get("TextTrackCue").copied());
    templates.insert("VTTCue", tmpl_vtt_cue);
    let tmpl_dedicated_worker_global_scope = super::web_apis::create_dedicated_worker_global_scope_template(scope, templates.get("WorkerGlobalScope").copied());
    templates.insert("DedicatedWorkerGlobalScope", tmpl_dedicated_worker_global_scope);
    let tmpl_rtc_identity_provider_global_scope = super::webrtc::create_rtc_identity_provider_global_scope_template(scope, templates.get("WorkerGlobalScope").copied());
    templates.insert("RTCIdentityProviderGlobalScope", tmpl_rtc_identity_provider_global_scope);
    let tmpl_service_worker_global_scope = super::workers::create_service_worker_global_scope_template(scope, templates.get("WorkerGlobalScope").copied());
    templates.insert("ServiceWorkerGlobalScope", tmpl_service_worker_global_scope);
    let tmpl_shared_worker_global_scope = super::web_apis::create_shared_worker_global_scope_template(scope, templates.get("WorkerGlobalScope").copied());
    templates.insert("SharedWorkerGlobalScope", tmpl_shared_worker_global_scope);
    let tmpl_xml_http_request = super::web_apis::create_xml_http_request_template(scope, templates.get("XMLHttpRequestEventTarget").copied());
    templates.insert("XMLHttpRequest", tmpl_xml_http_request);
    let tmpl_xml_http_request_upload = super::web_apis::create_xml_http_request_upload_template(scope, templates.get("XMLHttpRequestEventTarget").copied());
    templates.insert("XMLHttpRequestUpload", tmpl_xml_http_request_upload);
    let tmpl_xr_composition_layer = super::webxr::create_xr_composition_layer_template(scope, templates.get("XRLayer").copied());
    templates.insert("XRCompositionLayer", tmpl_xr_composition_layer);
    let tmpl_xr_web_gl_layer = super::webxr::create_xr_web_gl_layer_template(scope, templates.get("XRLayer").copied());
    templates.insert("XRWebGLLayer", tmpl_xr_web_gl_layer);
    let tmpl_xr_body_space = super::webxr::create_xr_body_space_template(scope, templates.get("XRSpace").copied());
    templates.insert("XRBodySpace", tmpl_xr_body_space);
    let tmpl_xr_joint_space = super::webxr::create_xr_joint_space_template(scope, templates.get("XRSpace").copied());
    templates.insert("XRJointSpace", tmpl_xr_joint_space);
    let tmpl_xr_reference_space = super::webxr::create_xr_reference_space_template(scope, templates.get("XRSpace").copied());
    templates.insert("XRReferenceSpace", tmpl_xr_reference_space);
    let tmpl_performance_navigation_timing = super::web_apis::create_performance_navigation_timing_template(scope, templates.get("PerformanceResourceTiming").copied());
    templates.insert("PerformanceNavigationTiming", tmpl_performance_navigation_timing);
    let tmpl_css_container_rule = super::css_om::create_css_container_rule_template(scope, templates.get("CSSConditionRule").copied());
    templates.insert("CSSContainerRule", tmpl_css_container_rule);
    let tmpl_css_media_rule = super::css_om::create_css_media_rule_template(scope, templates.get("CSSConditionRule").copied());
    templates.insert("CSSMediaRule", tmpl_css_media_rule);
    let tmpl_css_supports_rule = super::css_om::create_css_supports_rule_template(scope, templates.get("CSSConditionRule").copied());
    templates.insert("CSSSupportsRule", tmpl_css_supports_rule);
    let tmpl_css_math_clamp = super::css_om::create_css_math_clamp_template(scope, templates.get("CSSMathValue").copied());
    templates.insert("CSSMathClamp", tmpl_css_math_clamp);
    let tmpl_css_math_invert = super::css_om::create_css_math_invert_template(scope, templates.get("CSSMathValue").copied());
    templates.insert("CSSMathInvert", tmpl_css_math_invert);
    let tmpl_css_math_max = super::css_om::create_css_math_max_template(scope, templates.get("CSSMathValue").copied());
    templates.insert("CSSMathMax", tmpl_css_math_max);
    let tmpl_css_math_min = super::css_om::create_css_math_min_template(scope, templates.get("CSSMathValue").copied());
    templates.insert("CSSMathMin", tmpl_css_math_min);
    let tmpl_css_math_negate = super::css_om::create_css_math_negate_template(scope, templates.get("CSSMathValue").copied());
    templates.insert("CSSMathNegate", tmpl_css_math_negate);
    let tmpl_css_math_product = super::css_om::create_css_math_product_template(scope, templates.get("CSSMathValue").copied());
    templates.insert("CSSMathProduct", tmpl_css_math_product);
    let tmpl_css_math_sum = super::css_om::create_css_math_sum_template(scope, templates.get("CSSMathValue").copied());
    templates.insert("CSSMathSum", tmpl_css_math_sum);
    let tmpl_background_fetch_update_ui_event = super::events::create_background_fetch_update_ui_event_template(scope, templates.get("BackgroundFetchEvent").copied());
    templates.insert("BackgroundFetchUpdateUIEvent", tmpl_background_fetch_update_ui_event);
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
    let tmpl_xml_document = super::web_apis::create_xml_document_template(scope, templates.get("Document").copied());
    templates.insert("XMLDocument", tmpl_xml_document);
    let tmpl_shadow_root = super::dom_core::create_shadow_root_template(scope, templates.get("DocumentFragment").copied());
    templates.insert("ShadowRoot", tmpl_shadow_root);
    let tmpl_html_element = super::html_elements::create_html_element_template(scope, templates.get("Element").copied());
    templates.insert("HTMLElement", tmpl_html_element);
    let tmpl_math_ml_element = super::web_apis::create_math_ml_element_template(scope, templates.get("Element").copied());
    templates.insert("MathMLElement", tmpl_math_ml_element);
    let tmpl_svg_element = super::svg::create_svg_element_template(scope, templates.get("Element").copied());
    templates.insert("SVGElement", tmpl_svg_element);
    let tmpl_gravity_sensor = super::sensors::create_gravity_sensor_template(scope, templates.get("Accelerometer").copied());
    templates.insert("GravitySensor", tmpl_gravity_sensor);
    let tmpl_linear_acceleration_sensor = super::sensors::create_linear_acceleration_sensor_template(scope, templates.get("Accelerometer").copied());
    templates.insert("LinearAccelerationSensor", tmpl_linear_acceleration_sensor);
    let tmpl_absolute_orientation_sensor = super::sensors::create_absolute_orientation_sensor_template(scope, templates.get("OrientationSensor").copied());
    templates.insert("AbsoluteOrientationSensor", tmpl_absolute_orientation_sensor);
    let tmpl_relative_orientation_sensor = super::sensors::create_relative_orientation_sensor_template(scope, templates.get("OrientationSensor").copied());
    templates.insert("RelativeOrientationSensor", tmpl_relative_orientation_sensor);
    let tmpl_xr_cube_layer = super::webxr::create_xr_cube_layer_template(scope, templates.get("XRCompositionLayer").copied());
    templates.insert("XRCubeLayer", tmpl_xr_cube_layer);
    let tmpl_xr_cylinder_layer = super::webxr::create_xr_cylinder_layer_template(scope, templates.get("XRCompositionLayer").copied());
    templates.insert("XRCylinderLayer", tmpl_xr_cylinder_layer);
    let tmpl_xr_equirect_layer = super::webxr::create_xr_equirect_layer_template(scope, templates.get("XRCompositionLayer").copied());
    templates.insert("XREquirectLayer", tmpl_xr_equirect_layer);
    let tmpl_xr_projection_layer = super::webxr::create_xr_projection_layer_template(scope, templates.get("XRCompositionLayer").copied());
    templates.insert("XRProjectionLayer", tmpl_xr_projection_layer);
    let tmpl_xr_quad_layer = super::webxr::create_xr_quad_layer_template(scope, templates.get("XRCompositionLayer").copied());
    templates.insert("XRQuadLayer", tmpl_xr_quad_layer);
    let tmpl_xr_bounded_reference_space = super::webxr::create_xr_bounded_reference_space_template(scope, templates.get("XRReferenceSpace").copied());
    templates.insert("XRBoundedReferenceSpace", tmpl_xr_bounded_reference_space);
    let tmpl_cdata_section = super::web_apis::create_cdata_section_template(scope, templates.get("Text").copied());
    templates.insert("CDATASection", tmpl_cdata_section);
    let tmpl_svg_use_element_shadow_root = super::svg::create_svg_use_element_shadow_root_template(scope, templates.get("ShadowRoot").copied());
    templates.insert("SVGUseElementShadowRoot", tmpl_svg_use_element_shadow_root);
    let tmpl_html_anchor_element = super::html_elements::create_html_anchor_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLAnchorElement", tmpl_html_anchor_element);
    let tmpl_html_area_element = super::html_elements::create_html_area_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLAreaElement", tmpl_html_area_element);
    let tmpl_htmlbr_element = super::html_elements::create_htmlbr_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLBRElement", tmpl_htmlbr_element);
    let tmpl_html_base_element = super::html_elements::create_html_base_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLBaseElement", tmpl_html_base_element);
    let tmpl_html_body_element = super::html_elements::create_html_body_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLBodyElement", tmpl_html_body_element);
    let tmpl_html_button_element = super::html_elements::create_html_button_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLButtonElement", tmpl_html_button_element);
    let tmpl_html_canvas_element = super::html_elements::create_html_canvas_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLCanvasElement", tmpl_html_canvas_element);
    let tmpl_htmld_list_element = super::html_elements::create_htmld_list_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLDListElement", tmpl_htmld_list_element);
    let tmpl_html_data_element = super::html_elements::create_html_data_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLDataElement", tmpl_html_data_element);
    let tmpl_html_data_list_element = super::html_elements::create_html_data_list_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLDataListElement", tmpl_html_data_list_element);
    let tmpl_html_details_element = super::html_elements::create_html_details_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLDetailsElement", tmpl_html_details_element);
    let tmpl_html_dialog_element = super::html_elements::create_html_dialog_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLDialogElement", tmpl_html_dialog_element);
    let tmpl_html_directory_element = super::html_elements::create_html_directory_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLDirectoryElement", tmpl_html_directory_element);
    let tmpl_html_div_element = super::html_elements::create_html_div_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLDivElement", tmpl_html_div_element);
    let tmpl_html_embed_element = super::html_elements::create_html_embed_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLEmbedElement", tmpl_html_embed_element);
    let tmpl_html_fenced_frame_element = super::html_elements::create_html_fenced_frame_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLFencedFrameElement", tmpl_html_fenced_frame_element);
    let tmpl_html_field_set_element = super::html_elements::create_html_field_set_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLFieldSetElement", tmpl_html_field_set_element);
    let tmpl_html_font_element = super::html_elements::create_html_font_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLFontElement", tmpl_html_font_element);
    let tmpl_html_form_element = super::html_elements::create_html_form_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLFormElement", tmpl_html_form_element);
    let tmpl_html_frame_element = super::html_elements::create_html_frame_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLFrameElement", tmpl_html_frame_element);
    let tmpl_html_frame_set_element = super::html_elements::create_html_frame_set_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLFrameSetElement", tmpl_html_frame_set_element);
    let tmpl_html_geolocation_element = super::html_elements::create_html_geolocation_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLGeolocationElement", tmpl_html_geolocation_element);
    let tmpl_htmlhr_element = super::html_elements::create_htmlhr_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLHRElement", tmpl_htmlhr_element);
    let tmpl_html_head_element = super::html_elements::create_html_head_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLHeadElement", tmpl_html_head_element);
    let tmpl_html_heading_element = super::html_elements::create_html_heading_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLHeadingElement", tmpl_html_heading_element);
    let tmpl_html_html_element = super::html_elements::create_html_html_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLHtmlElement", tmpl_html_html_element);
    let tmpl_htmli_frame_element = super::html_elements::create_htmli_frame_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLIFrameElement", tmpl_htmli_frame_element);
    let tmpl_html_image_element = super::html_elements::create_html_image_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLImageElement", tmpl_html_image_element);
    let tmpl_html_input_element = super::html_elements::create_html_input_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLInputElement", tmpl_html_input_element);
    let tmpl_htmlli_element = super::html_elements::create_htmlli_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLLIElement", tmpl_htmlli_element);
    let tmpl_html_label_element = super::html_elements::create_html_label_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLLabelElement", tmpl_html_label_element);
    let tmpl_html_legend_element = super::html_elements::create_html_legend_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLLegendElement", tmpl_html_legend_element);
    let tmpl_html_link_element = super::html_elements::create_html_link_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLLinkElement", tmpl_html_link_element);
    let tmpl_html_map_element = super::html_elements::create_html_map_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLMapElement", tmpl_html_map_element);
    let tmpl_html_marquee_element = super::html_elements::create_html_marquee_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLMarqueeElement", tmpl_html_marquee_element);
    let tmpl_html_media_element = super::html_elements::create_html_media_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLMediaElement", tmpl_html_media_element);
    let tmpl_html_menu_element = super::html_elements::create_html_menu_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLMenuElement", tmpl_html_menu_element);
    let tmpl_html_meta_element = super::html_elements::create_html_meta_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLMetaElement", tmpl_html_meta_element);
    let tmpl_html_meter_element = super::html_elements::create_html_meter_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLMeterElement", tmpl_html_meter_element);
    let tmpl_html_mod_element = super::html_elements::create_html_mod_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLModElement", tmpl_html_mod_element);
    let tmpl_html_model_element = super::html_elements::create_html_model_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLModelElement", tmpl_html_model_element);
    let tmpl_htmlo_list_element = super::html_elements::create_htmlo_list_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLOListElement", tmpl_htmlo_list_element);
    let tmpl_html_object_element = super::html_elements::create_html_object_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLObjectElement", tmpl_html_object_element);
    let tmpl_html_opt_group_element = super::html_elements::create_html_opt_group_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLOptGroupElement", tmpl_html_opt_group_element);
    let tmpl_html_option_element = super::html_elements::create_html_option_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLOptionElement", tmpl_html_option_element);
    let tmpl_html_output_element = super::html_elements::create_html_output_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLOutputElement", tmpl_html_output_element);
    let tmpl_html_paragraph_element = super::html_elements::create_html_paragraph_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLParagraphElement", tmpl_html_paragraph_element);
    let tmpl_html_param_element = super::html_elements::create_html_param_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLParamElement", tmpl_html_param_element);
    let tmpl_html_picture_element = super::html_elements::create_html_picture_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLPictureElement", tmpl_html_picture_element);
    let tmpl_html_portal_element = super::html_elements::create_html_portal_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLPortalElement", tmpl_html_portal_element);
    let tmpl_html_pre_element = super::html_elements::create_html_pre_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLPreElement", tmpl_html_pre_element);
    let tmpl_html_progress_element = super::html_elements::create_html_progress_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLProgressElement", tmpl_html_progress_element);
    let tmpl_html_quote_element = super::html_elements::create_html_quote_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLQuoteElement", tmpl_html_quote_element);
    let tmpl_html_script_element = super::html_elements::create_html_script_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLScriptElement", tmpl_html_script_element);
    let tmpl_html_select_element = super::html_elements::create_html_select_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLSelectElement", tmpl_html_select_element);
    let tmpl_html_selected_content_element = super::html_elements::create_html_selected_content_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLSelectedContentElement", tmpl_html_selected_content_element);
    let tmpl_html_slot_element = super::html_elements::create_html_slot_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLSlotElement", tmpl_html_slot_element);
    let tmpl_html_source_element = super::html_elements::create_html_source_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLSourceElement", tmpl_html_source_element);
    let tmpl_html_span_element = super::html_elements::create_html_span_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLSpanElement", tmpl_html_span_element);
    let tmpl_html_style_element = super::html_elements::create_html_style_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLStyleElement", tmpl_html_style_element);
    let tmpl_html_table_caption_element = super::html_elements::create_html_table_caption_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLTableCaptionElement", tmpl_html_table_caption_element);
    let tmpl_html_table_cell_element = super::html_elements::create_html_table_cell_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLTableCellElement", tmpl_html_table_cell_element);
    let tmpl_html_table_col_element = super::html_elements::create_html_table_col_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLTableColElement", tmpl_html_table_col_element);
    let tmpl_html_table_element = super::html_elements::create_html_table_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLTableElement", tmpl_html_table_element);
    let tmpl_html_table_row_element = super::html_elements::create_html_table_row_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLTableRowElement", tmpl_html_table_row_element);
    let tmpl_html_table_section_element = super::html_elements::create_html_table_section_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLTableSectionElement", tmpl_html_table_section_element);
    let tmpl_html_template_element = super::html_elements::create_html_template_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLTemplateElement", tmpl_html_template_element);
    let tmpl_html_text_area_element = super::html_elements::create_html_text_area_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLTextAreaElement", tmpl_html_text_area_element);
    let tmpl_html_time_element = super::html_elements::create_html_time_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLTimeElement", tmpl_html_time_element);
    let tmpl_html_title_element = super::html_elements::create_html_title_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLTitleElement", tmpl_html_title_element);
    let tmpl_html_track_element = super::html_elements::create_html_track_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLTrackElement", tmpl_html_track_element);
    let tmpl_htmlu_list_element = super::html_elements::create_htmlu_list_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLUListElement", tmpl_htmlu_list_element);
    let tmpl_html_unknown_element = super::html_elements::create_html_unknown_element_template(scope, templates.get("HTMLElement").copied());
    templates.insert("HTMLUnknownElement", tmpl_html_unknown_element);
    let tmpl_math_ml_anchor_element = super::web_apis::create_math_ml_anchor_element_template(scope, templates.get("MathMLElement").copied());
    templates.insert("MathMLAnchorElement", tmpl_math_ml_anchor_element);
    let tmpl_svg_animation_element = super::svg::create_svg_animation_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGAnimationElement", tmpl_svg_animation_element);
    let tmpl_svg_clip_path_element = super::svg::create_svg_clip_path_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGClipPathElement", tmpl_svg_clip_path_element);
    let tmpl_svg_component_transfer_function_element = super::svg::create_svg_component_transfer_function_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGComponentTransferFunctionElement", tmpl_svg_component_transfer_function_element);
    let tmpl_svg_desc_element = super::svg::create_svg_desc_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGDescElement", tmpl_svg_desc_element);
    let tmpl_svgfe_blend_element = super::svg::create_svgfe_blend_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEBlendElement", tmpl_svgfe_blend_element);
    let tmpl_svgfe_color_matrix_element = super::svg::create_svgfe_color_matrix_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEColorMatrixElement", tmpl_svgfe_color_matrix_element);
    let tmpl_svgfe_component_transfer_element = super::svg::create_svgfe_component_transfer_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEComponentTransferElement", tmpl_svgfe_component_transfer_element);
    let tmpl_svgfe_composite_element = super::svg::create_svgfe_composite_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFECompositeElement", tmpl_svgfe_composite_element);
    let tmpl_svgfe_convolve_matrix_element = super::svg::create_svgfe_convolve_matrix_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEConvolveMatrixElement", tmpl_svgfe_convolve_matrix_element);
    let tmpl_svgfe_diffuse_lighting_element = super::svg::create_svgfe_diffuse_lighting_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEDiffuseLightingElement", tmpl_svgfe_diffuse_lighting_element);
    let tmpl_svgfe_displacement_map_element = super::svg::create_svgfe_displacement_map_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEDisplacementMapElement", tmpl_svgfe_displacement_map_element);
    let tmpl_svgfe_distant_light_element = super::svg::create_svgfe_distant_light_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEDistantLightElement", tmpl_svgfe_distant_light_element);
    let tmpl_svgfe_drop_shadow_element = super::svg::create_svgfe_drop_shadow_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEDropShadowElement", tmpl_svgfe_drop_shadow_element);
    let tmpl_svgfe_flood_element = super::svg::create_svgfe_flood_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEFloodElement", tmpl_svgfe_flood_element);
    let tmpl_svgfe_gaussian_blur_element = super::svg::create_svgfe_gaussian_blur_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEGaussianBlurElement", tmpl_svgfe_gaussian_blur_element);
    let tmpl_svgfe_image_element = super::svg::create_svgfe_image_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEImageElement", tmpl_svgfe_image_element);
    let tmpl_svgfe_merge_element = super::svg::create_svgfe_merge_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEMergeElement", tmpl_svgfe_merge_element);
    let tmpl_svgfe_merge_node_element = super::svg::create_svgfe_merge_node_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEMergeNodeElement", tmpl_svgfe_merge_node_element);
    let tmpl_svgfe_morphology_element = super::svg::create_svgfe_morphology_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEMorphologyElement", tmpl_svgfe_morphology_element);
    let tmpl_svgfe_offset_element = super::svg::create_svgfe_offset_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEOffsetElement", tmpl_svgfe_offset_element);
    let tmpl_svgfe_point_light_element = super::svg::create_svgfe_point_light_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFEPointLightElement", tmpl_svgfe_point_light_element);
    let tmpl_svgfe_specular_lighting_element = super::svg::create_svgfe_specular_lighting_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFESpecularLightingElement", tmpl_svgfe_specular_lighting_element);
    let tmpl_svgfe_spot_light_element = super::svg::create_svgfe_spot_light_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFESpotLightElement", tmpl_svgfe_spot_light_element);
    let tmpl_svgfe_tile_element = super::svg::create_svgfe_tile_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFETileElement", tmpl_svgfe_tile_element);
    let tmpl_svgfe_turbulence_element = super::svg::create_svgfe_turbulence_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFETurbulenceElement", tmpl_svgfe_turbulence_element);
    let tmpl_svg_filter_element = super::svg::create_svg_filter_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGFilterElement", tmpl_svg_filter_element);
    let tmpl_svg_gradient_element = super::svg::create_svg_gradient_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGGradientElement", tmpl_svg_gradient_element);
    let tmpl_svg_graphics_element = super::svg::create_svg_graphics_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGGraphicsElement", tmpl_svg_graphics_element);
    let tmpl_svgm_path_element = super::svg::create_svgm_path_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGMPathElement", tmpl_svgm_path_element);
    let tmpl_svg_marker_element = super::svg::create_svg_marker_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGMarkerElement", tmpl_svg_marker_element);
    let tmpl_svg_mask_element = super::svg::create_svg_mask_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGMaskElement", tmpl_svg_mask_element);
    let tmpl_svg_metadata_element = super::svg::create_svg_metadata_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGMetadataElement", tmpl_svg_metadata_element);
    let tmpl_svg_pattern_element = super::svg::create_svg_pattern_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGPatternElement", tmpl_svg_pattern_element);
    let tmpl_svg_script_element = super::svg::create_svg_script_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGScriptElement", tmpl_svg_script_element);
    let tmpl_svg_stop_element = super::svg::create_svg_stop_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGStopElement", tmpl_svg_stop_element);
    let tmpl_svg_style_element = super::svg::create_svg_style_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGStyleElement", tmpl_svg_style_element);
    let tmpl_svg_title_element = super::svg::create_svg_title_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGTitleElement", tmpl_svg_title_element);
    let tmpl_svg_view_element = super::svg::create_svg_view_element_template(scope, templates.get("SVGElement").copied());
    templates.insert("SVGViewElement", tmpl_svg_view_element);
    let tmpl_html_audio_element = super::html_elements::create_html_audio_element_template(scope, templates.get("HTMLMediaElement").copied());
    templates.insert("HTMLAudioElement", tmpl_html_audio_element);
    let tmpl_html_video_element = super::html_elements::create_html_video_element_template(scope, templates.get("HTMLMediaElement").copied());
    templates.insert("HTMLVideoElement", tmpl_html_video_element);
    let tmpl_svg_animate_element = super::svg::create_svg_animate_element_template(scope, templates.get("SVGAnimationElement").copied());
    templates.insert("SVGAnimateElement", tmpl_svg_animate_element);
    let tmpl_svg_animate_motion_element = super::svg::create_svg_animate_motion_element_template(scope, templates.get("SVGAnimationElement").copied());
    templates.insert("SVGAnimateMotionElement", tmpl_svg_animate_motion_element);
    let tmpl_svg_animate_transform_element = super::svg::create_svg_animate_transform_element_template(scope, templates.get("SVGAnimationElement").copied());
    templates.insert("SVGAnimateTransformElement", tmpl_svg_animate_transform_element);
    let tmpl_svg_set_element = super::svg::create_svg_set_element_template(scope, templates.get("SVGAnimationElement").copied());
    templates.insert("SVGSetElement", tmpl_svg_set_element);
    let tmpl_svgfe_func_a_element = super::svg::create_svgfe_func_a_element_template(scope, templates.get("SVGComponentTransferFunctionElement").copied());
    templates.insert("SVGFEFuncAElement", tmpl_svgfe_func_a_element);
    let tmpl_svgfe_func_b_element = super::svg::create_svgfe_func_b_element_template(scope, templates.get("SVGComponentTransferFunctionElement").copied());
    templates.insert("SVGFEFuncBElement", tmpl_svgfe_func_b_element);
    let tmpl_svgfe_func_g_element = super::svg::create_svgfe_func_g_element_template(scope, templates.get("SVGComponentTransferFunctionElement").copied());
    templates.insert("SVGFEFuncGElement", tmpl_svgfe_func_g_element);
    let tmpl_svgfe_func_r_element = super::svg::create_svgfe_func_r_element_template(scope, templates.get("SVGComponentTransferFunctionElement").copied());
    templates.insert("SVGFEFuncRElement", tmpl_svgfe_func_r_element);
    let tmpl_svg_linear_gradient_element = super::svg::create_svg_linear_gradient_element_template(scope, templates.get("SVGGradientElement").copied());
    templates.insert("SVGLinearGradientElement", tmpl_svg_linear_gradient_element);
    let tmpl_svg_radial_gradient_element = super::svg::create_svg_radial_gradient_element_template(scope, templates.get("SVGGradientElement").copied());
    templates.insert("SVGRadialGradientElement", tmpl_svg_radial_gradient_element);
    let tmpl_svga_element = super::svg::create_svga_element_template(scope, templates.get("SVGGraphicsElement").copied());
    templates.insert("SVGAElement", tmpl_svga_element);
    let tmpl_svg_defs_element = super::svg::create_svg_defs_element_template(scope, templates.get("SVGGraphicsElement").copied());
    templates.insert("SVGDefsElement", tmpl_svg_defs_element);
    let tmpl_svg_foreign_object_element = super::svg::create_svg_foreign_object_element_template(scope, templates.get("SVGGraphicsElement").copied());
    templates.insert("SVGForeignObjectElement", tmpl_svg_foreign_object_element);
    let tmpl_svgg_element = super::svg::create_svgg_element_template(scope, templates.get("SVGGraphicsElement").copied());
    templates.insert("SVGGElement", tmpl_svgg_element);
    let tmpl_svg_geometry_element = super::svg::create_svg_geometry_element_template(scope, templates.get("SVGGraphicsElement").copied());
    templates.insert("SVGGeometryElement", tmpl_svg_geometry_element);
    let tmpl_svg_image_element = super::svg::create_svg_image_element_template(scope, templates.get("SVGGraphicsElement").copied());
    templates.insert("SVGImageElement", tmpl_svg_image_element);
    let tmpl_svgsvg_element = super::svg::create_svgsvg_element_template(scope, templates.get("SVGGraphicsElement").copied());
    templates.insert("SVGSVGElement", tmpl_svgsvg_element);
    let tmpl_svg_switch_element = super::svg::create_svg_switch_element_template(scope, templates.get("SVGGraphicsElement").copied());
    templates.insert("SVGSwitchElement", tmpl_svg_switch_element);
    let tmpl_svg_symbol_element = super::svg::create_svg_symbol_element_template(scope, templates.get("SVGGraphicsElement").copied());
    templates.insert("SVGSymbolElement", tmpl_svg_symbol_element);
    let tmpl_svg_text_content_element = super::svg::create_svg_text_content_element_template(scope, templates.get("SVGGraphicsElement").copied());
    templates.insert("SVGTextContentElement", tmpl_svg_text_content_element);
    let tmpl_svg_use_element = super::svg::create_svg_use_element_template(scope, templates.get("SVGGraphicsElement").copied());
    templates.insert("SVGUseElement", tmpl_svg_use_element);
    let tmpl_svg_circle_element = super::svg::create_svg_circle_element_template(scope, templates.get("SVGGeometryElement").copied());
    templates.insert("SVGCircleElement", tmpl_svg_circle_element);
    let tmpl_svg_ellipse_element = super::svg::create_svg_ellipse_element_template(scope, templates.get("SVGGeometryElement").copied());
    templates.insert("SVGEllipseElement", tmpl_svg_ellipse_element);
    let tmpl_svg_line_element = super::svg::create_svg_line_element_template(scope, templates.get("SVGGeometryElement").copied());
    templates.insert("SVGLineElement", tmpl_svg_line_element);
    let tmpl_svg_path_element = super::svg::create_svg_path_element_template(scope, templates.get("SVGGeometryElement").copied());
    templates.insert("SVGPathElement", tmpl_svg_path_element);
    let tmpl_svg_polygon_element = super::svg::create_svg_polygon_element_template(scope, templates.get("SVGGeometryElement").copied());
    templates.insert("SVGPolygonElement", tmpl_svg_polygon_element);
    let tmpl_svg_polyline_element = super::svg::create_svg_polyline_element_template(scope, templates.get("SVGGeometryElement").copied());
    templates.insert("SVGPolylineElement", tmpl_svg_polyline_element);
    let tmpl_svg_rect_element = super::svg::create_svg_rect_element_template(scope, templates.get("SVGGeometryElement").copied());
    templates.insert("SVGRectElement", tmpl_svg_rect_element);
    let tmpl_svg_text_path_element = super::svg::create_svg_text_path_element_template(scope, templates.get("SVGTextContentElement").copied());
    templates.insert("SVGTextPathElement", tmpl_svg_text_path_element);
    let tmpl_svg_text_positioning_element = super::svg::create_svg_text_positioning_element_template(scope, templates.get("SVGTextContentElement").copied());
    templates.insert("SVGTextPositioningElement", tmpl_svg_text_positioning_element);
    let tmpl_svgt_span_element = super::svg::create_svgt_span_element_template(scope, templates.get("SVGTextPositioningElement").copied());
    templates.insert("SVGTSpanElement", tmpl_svgt_span_element);
    let tmpl_svg_text_element = super::svg::create_svg_text_element_template(scope, templates.get("SVGTextPositioningElement").copied());
    templates.insert("SVGTextElement", tmpl_svg_text_element);

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
    if let Some(ctor_bluetooth_le_scan) = tmpl_bluetooth_le_scan.get_function(scope) {
        let name_bluetooth_le_scan = v8::String::new(scope, "BluetoothLEScan").unwrap();
        global.define_own_property(scope, name_bluetooth_le_scan.into(), ctor_bluetooth_le_scan.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_le_scan_filter) = tmpl_bluetooth_le_scan_filter.get_function(scope) {
        let name_bluetooth_le_scan_filter = v8::String::new(scope, "BluetoothLEScanFilter").unwrap();
        global.define_own_property(scope, name_bluetooth_le_scan_filter.into(), ctor_bluetooth_le_scan_filter.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_manufacturer_data_filter) = tmpl_bluetooth_manufacturer_data_filter.get_function(scope) {
        let name_bluetooth_manufacturer_data_filter = v8::String::new(scope, "BluetoothManufacturerDataFilter").unwrap();
        global.define_own_property(scope, name_bluetooth_manufacturer_data_filter.into(), ctor_bluetooth_manufacturer_data_filter.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_manufacturer_data_map) = tmpl_bluetooth_manufacturer_data_map.get_function(scope) {
        let name_bluetooth_manufacturer_data_map = v8::String::new(scope, "BluetoothManufacturerDataMap").unwrap();
        global.define_own_property(scope, name_bluetooth_manufacturer_data_map.into(), ctor_bluetooth_manufacturer_data_map.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_remote_gatt_descriptor) = tmpl_bluetooth_remote_gatt_descriptor.get_function(scope) {
        let name_bluetooth_remote_gatt_descriptor = v8::String::new(scope, "BluetoothRemoteGATTDescriptor").unwrap();
        global.define_own_property(scope, name_bluetooth_remote_gatt_descriptor.into(), ctor_bluetooth_remote_gatt_descriptor.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_remote_gatt_server) = tmpl_bluetooth_remote_gatt_server.get_function(scope) {
        let name_bluetooth_remote_gatt_server = v8::String::new(scope, "BluetoothRemoteGATTServer").unwrap();
        global.define_own_property(scope, name_bluetooth_remote_gatt_server.into(), ctor_bluetooth_remote_gatt_server.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_css_font_feature_values_map) = tmpl_css_font_feature_values_map.get_function(scope) {
        let name_css_font_feature_values_map = v8::String::new(scope, "CSSFontFeatureValuesMap").unwrap();
        global.define_own_property(scope, name_css_font_feature_values_map.into(), ctor_css_font_feature_values_map.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_numeric_array) = tmpl_css_numeric_array.get_function(scope) {
        let name_css_numeric_array = v8::String::new(scope, "CSSNumericArray").unwrap();
        global.define_own_property(scope, name_css_numeric_array.into(), ctor_css_numeric_array.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_parser_rule) = tmpl_css_parser_rule.get_function(scope) {
        let name_css_parser_rule = v8::String::new(scope, "CSSParserRule").unwrap();
        global.define_own_property(scope, name_css_parser_rule.into(), ctor_css_parser_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_parser_value) = tmpl_css_parser_value.get_function(scope) {
        let name_css_parser_value = v8::String::new(scope, "CSSParserValue").unwrap();
        global.define_own_property(scope, name_css_parser_value.into(), ctor_css_parser_value.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_pseudo_element) = tmpl_css_pseudo_element.get_function(scope) {
        let name_css_pseudo_element = v8::String::new(scope, "CSSPseudoElement").unwrap();
        global.define_own_property(scope, name_css_pseudo_element.into(), ctor_css_pseudo_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_rule) = tmpl_css_rule.get_function(scope) {
        let name_css_rule = v8::String::new(scope, "CSSRule").unwrap();
        global.define_own_property(scope, name_css_rule.into(), ctor_css_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_rule_list) = tmpl_css_rule_list.get_function(scope) {
        let name_css_rule_list = v8::String::new(scope, "CSSRuleList").unwrap();
        global.define_own_property(scope, name_css_rule_list.into(), ctor_css_rule_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_style_declaration) = tmpl_css_style_declaration.get_function(scope) {
        let name_css_style_declaration = v8::String::new(scope, "CSSStyleDeclaration").unwrap();
        global.define_own_property(scope, name_css_style_declaration.into(), ctor_css_style_declaration.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_style_value) = tmpl_css_style_value.get_function(scope) {
        let name_css_style_value = v8::String::new(scope, "CSSStyleValue").unwrap();
        global.define_own_property(scope, name_css_style_value.into(), ctor_css_style_value.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_transform_component) = tmpl_css_transform_component.get_function(scope) {
        let name_css_transform_component = v8::String::new(scope, "CSSTransformComponent").unwrap();
        global.define_own_property(scope, name_css_transform_component.into(), ctor_css_transform_component.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_variable_reference_value) = tmpl_css_variable_reference_value.get_function(scope) {
        let name_css_variable_reference_value = v8::String::new(scope, "CSSVariableReferenceValue").unwrap();
        global.define_own_property(scope, name_css_variable_reference_value.into(), ctor_css_variable_reference_value.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_canvas_rendering_context2d) = tmpl_canvas_rendering_context2d.get_function(scope) {
        let name_canvas_rendering_context2d = v8::String::new(scope, "CanvasRenderingContext2D").unwrap();
        global.define_own_property(scope, name_canvas_rendering_context2d.into(), ctor_canvas_rendering_context2d.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_dom_exception) = tmpl_dom_exception.get_function(scope) {
        let name_dom_exception = v8::String::new(scope, "DOMException").unwrap();
        global.define_own_property(scope, name_dom_exception.into(), ctor_dom_exception.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_dom_implementation) = tmpl_dom_implementation.get_function(scope) {
        let name_dom_implementation = v8::String::new(scope, "DOMImplementation").unwrap();
        global.define_own_property(scope, name_dom_implementation.into(), ctor_dom_implementation.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_dom_matrix_read_only) = tmpl_dom_matrix_read_only.get_function(scope) {
        let name_dom_matrix_read_only = v8::String::new(scope, "DOMMatrixReadOnly").unwrap();
        global.define_own_property(scope, name_dom_matrix_read_only.into(), ctor_dom_matrix_read_only.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_dom_parser) = tmpl_dom_parser.get_function(scope) {
        let name_dom_parser = v8::String::new(scope, "DOMParser").unwrap();
        global.define_own_property(scope, name_dom_parser.into(), ctor_dom_parser.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_dom_point_read_only) = tmpl_dom_point_read_only.get_function(scope) {
        let name_dom_point_read_only = v8::String::new(scope, "DOMPointReadOnly").unwrap();
        global.define_own_property(scope, name_dom_point_read_only.into(), ctor_dom_point_read_only.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_dom_quad) = tmpl_dom_quad.get_function(scope) {
        let name_dom_quad = v8::String::new(scope, "DOMQuad").unwrap();
        global.define_own_property(scope, name_dom_quad.into(), ctor_dom_quad.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_dom_rect_list) = tmpl_dom_rect_list.get_function(scope) {
        let name_dom_rect_list = v8::String::new(scope, "DOMRectList").unwrap();
        global.define_own_property(scope, name_dom_rect_list.into(), ctor_dom_rect_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_dom_rect_read_only) = tmpl_dom_rect_read_only.get_function(scope) {
        let name_dom_rect_read_only = v8::String::new(scope, "DOMRectReadOnly").unwrap();
        global.define_own_property(scope, name_dom_rect_read_only.into(), ctor_dom_rect_read_only.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_dom_string_list) = tmpl_dom_string_list.get_function(scope) {
        let name_dom_string_list = v8::String::new(scope, "DOMStringList").unwrap();
        global.define_own_property(scope, name_dom_string_list.into(), ctor_dom_string_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_dom_string_map) = tmpl_dom_string_map.get_function(scope) {
        let name_dom_string_map = v8::String::new(scope, "DOMStringMap").unwrap();
        global.define_own_property(scope, name_dom_string_map.into(), ctor_dom_string_map.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_dom_token_list) = tmpl_dom_token_list.get_function(scope) {
        let name_dom_token_list = v8::String::new(scope, "DOMTokenList").unwrap();
        global.define_own_property(scope, name_dom_token_list.into(), ctor_dom_token_list.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_gpu_adapter) = tmpl_gpu_adapter.get_function(scope) {
        let name_gpu_adapter = v8::String::new(scope, "GPUAdapter").unwrap();
        global.define_own_property(scope, name_gpu_adapter.into(), ctor_gpu_adapter.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_adapter_info) = tmpl_gpu_adapter_info.get_function(scope) {
        let name_gpu_adapter_info = v8::String::new(scope, "GPUAdapterInfo").unwrap();
        global.define_own_property(scope, name_gpu_adapter_info.into(), ctor_gpu_adapter_info.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_bind_group) = tmpl_gpu_bind_group.get_function(scope) {
        let name_gpu_bind_group = v8::String::new(scope, "GPUBindGroup").unwrap();
        global.define_own_property(scope, name_gpu_bind_group.into(), ctor_gpu_bind_group.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_bind_group_layout) = tmpl_gpu_bind_group_layout.get_function(scope) {
        let name_gpu_bind_group_layout = v8::String::new(scope, "GPUBindGroupLayout").unwrap();
        global.define_own_property(scope, name_gpu_bind_group_layout.into(), ctor_gpu_bind_group_layout.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_buffer) = tmpl_gpu_buffer.get_function(scope) {
        let name_gpu_buffer = v8::String::new(scope, "GPUBuffer").unwrap();
        global.define_own_property(scope, name_gpu_buffer.into(), ctor_gpu_buffer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_canvas_context) = tmpl_gpu_canvas_context.get_function(scope) {
        let name_gpu_canvas_context = v8::String::new(scope, "GPUCanvasContext").unwrap();
        global.define_own_property(scope, name_gpu_canvas_context.into(), ctor_gpu_canvas_context.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_command_buffer) = tmpl_gpu_command_buffer.get_function(scope) {
        let name_gpu_command_buffer = v8::String::new(scope, "GPUCommandBuffer").unwrap();
        global.define_own_property(scope, name_gpu_command_buffer.into(), ctor_gpu_command_buffer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_command_encoder) = tmpl_gpu_command_encoder.get_function(scope) {
        let name_gpu_command_encoder = v8::String::new(scope, "GPUCommandEncoder").unwrap();
        global.define_own_property(scope, name_gpu_command_encoder.into(), ctor_gpu_command_encoder.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_compilation_info) = tmpl_gpu_compilation_info.get_function(scope) {
        let name_gpu_compilation_info = v8::String::new(scope, "GPUCompilationInfo").unwrap();
        global.define_own_property(scope, name_gpu_compilation_info.into(), ctor_gpu_compilation_info.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_compilation_message) = tmpl_gpu_compilation_message.get_function(scope) {
        let name_gpu_compilation_message = v8::String::new(scope, "GPUCompilationMessage").unwrap();
        global.define_own_property(scope, name_gpu_compilation_message.into(), ctor_gpu_compilation_message.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_compute_pass_encoder) = tmpl_gpu_compute_pass_encoder.get_function(scope) {
        let name_gpu_compute_pass_encoder = v8::String::new(scope, "GPUComputePassEncoder").unwrap();
        global.define_own_property(scope, name_gpu_compute_pass_encoder.into(), ctor_gpu_compute_pass_encoder.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_compute_pipeline) = tmpl_gpu_compute_pipeline.get_function(scope) {
        let name_gpu_compute_pipeline = v8::String::new(scope, "GPUComputePipeline").unwrap();
        global.define_own_property(scope, name_gpu_compute_pipeline.into(), ctor_gpu_compute_pipeline.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_device_lost_info) = tmpl_gpu_device_lost_info.get_function(scope) {
        let name_gpu_device_lost_info = v8::String::new(scope, "GPUDeviceLostInfo").unwrap();
        global.define_own_property(scope, name_gpu_device_lost_info.into(), ctor_gpu_device_lost_info.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_error) = tmpl_gpu_error.get_function(scope) {
        let name_gpu_error = v8::String::new(scope, "GPUError").unwrap();
        global.define_own_property(scope, name_gpu_error.into(), ctor_gpu_error.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_external_texture) = tmpl_gpu_external_texture.get_function(scope) {
        let name_gpu_external_texture = v8::String::new(scope, "GPUExternalTexture").unwrap();
        global.define_own_property(scope, name_gpu_external_texture.into(), ctor_gpu_external_texture.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_pipeline_layout) = tmpl_gpu_pipeline_layout.get_function(scope) {
        let name_gpu_pipeline_layout = v8::String::new(scope, "GPUPipelineLayout").unwrap();
        global.define_own_property(scope, name_gpu_pipeline_layout.into(), ctor_gpu_pipeline_layout.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_query_set) = tmpl_gpu_query_set.get_function(scope) {
        let name_gpu_query_set = v8::String::new(scope, "GPUQuerySet").unwrap();
        global.define_own_property(scope, name_gpu_query_set.into(), ctor_gpu_query_set.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_queue) = tmpl_gpu_queue.get_function(scope) {
        let name_gpu_queue = v8::String::new(scope, "GPUQueue").unwrap();
        global.define_own_property(scope, name_gpu_queue.into(), ctor_gpu_queue.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_render_bundle) = tmpl_gpu_render_bundle.get_function(scope) {
        let name_gpu_render_bundle = v8::String::new(scope, "GPURenderBundle").unwrap();
        global.define_own_property(scope, name_gpu_render_bundle.into(), ctor_gpu_render_bundle.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_render_bundle_encoder) = tmpl_gpu_render_bundle_encoder.get_function(scope) {
        let name_gpu_render_bundle_encoder = v8::String::new(scope, "GPURenderBundleEncoder").unwrap();
        global.define_own_property(scope, name_gpu_render_bundle_encoder.into(), ctor_gpu_render_bundle_encoder.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_render_pass_encoder) = tmpl_gpu_render_pass_encoder.get_function(scope) {
        let name_gpu_render_pass_encoder = v8::String::new(scope, "GPURenderPassEncoder").unwrap();
        global.define_own_property(scope, name_gpu_render_pass_encoder.into(), ctor_gpu_render_pass_encoder.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_render_pipeline) = tmpl_gpu_render_pipeline.get_function(scope) {
        let name_gpu_render_pipeline = v8::String::new(scope, "GPURenderPipeline").unwrap();
        global.define_own_property(scope, name_gpu_render_pipeline.into(), ctor_gpu_render_pipeline.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_sampler) = tmpl_gpu_sampler.get_function(scope) {
        let name_gpu_sampler = v8::String::new(scope, "GPUSampler").unwrap();
        global.define_own_property(scope, name_gpu_sampler.into(), ctor_gpu_sampler.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_shader_module) = tmpl_gpu_shader_module.get_function(scope) {
        let name_gpu_shader_module = v8::String::new(scope, "GPUShaderModule").unwrap();
        global.define_own_property(scope, name_gpu_shader_module.into(), ctor_gpu_shader_module.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_supported_features) = tmpl_gpu_supported_features.get_function(scope) {
        let name_gpu_supported_features = v8::String::new(scope, "GPUSupportedFeatures").unwrap();
        global.define_own_property(scope, name_gpu_supported_features.into(), ctor_gpu_supported_features.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_supported_limits) = tmpl_gpu_supported_limits.get_function(scope) {
        let name_gpu_supported_limits = v8::String::new(scope, "GPUSupportedLimits").unwrap();
        global.define_own_property(scope, name_gpu_supported_limits.into(), ctor_gpu_supported_limits.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_texture) = tmpl_gpu_texture.get_function(scope) {
        let name_gpu_texture = v8::String::new(scope, "GPUTexture").unwrap();
        global.define_own_property(scope, name_gpu_texture.into(), ctor_gpu_texture.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_texture_view) = tmpl_gpu_texture_view.get_function(scope) {
        let name_gpu_texture_view = v8::String::new(scope, "GPUTextureView").unwrap();
        global.define_own_property(scope, name_gpu_texture_view.into(), ctor_gpu_texture_view.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_html_all_collection) = tmpl_html_all_collection.get_function(scope) {
        let name_html_all_collection = v8::String::new(scope, "HTMLAllCollection").unwrap();
        global.define_own_property(scope, name_html_all_collection.into(), ctor_html_all_collection.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_collection) = tmpl_html_collection.get_function(scope) {
        let name_html_collection = v8::String::new(scope, "HTMLCollection").unwrap();
        global.define_own_property(scope, name_html_collection.into(), ctor_html_collection.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_idb_cursor) = tmpl_idb_cursor.get_function(scope) {
        let name_idb_cursor = v8::String::new(scope, "IDBCursor").unwrap();
        global.define_own_property(scope, name_idb_cursor.into(), ctor_idb_cursor.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idb_factory) = tmpl_idb_factory.get_function(scope) {
        let name_idb_factory = v8::String::new(scope, "IDBFactory").unwrap();
        global.define_own_property(scope, name_idb_factory.into(), ctor_idb_factory.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idb_index) = tmpl_idb_index.get_function(scope) {
        let name_idb_index = v8::String::new(scope, "IDBIndex").unwrap();
        global.define_own_property(scope, name_idb_index.into(), ctor_idb_index.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idb_key_range) = tmpl_idb_key_range.get_function(scope) {
        let name_idb_key_range = v8::String::new(scope, "IDBKeyRange").unwrap();
        global.define_own_property(scope, name_idb_key_range.into(), ctor_idb_key_range.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idb_object_store) = tmpl_idb_object_store.get_function(scope) {
        let name_idb_object_store = v8::String::new(scope, "IDBObjectStore").unwrap();
        global.define_own_property(scope, name_idb_object_store.into(), ctor_idb_object_store.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idb_record) = tmpl_idb_record.get_function(scope) {
        let name_idb_record = v8::String::new(scope, "IDBRecord").unwrap();
        global.define_own_property(scope, name_idb_record.into(), ctor_idb_record.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_midi_input_map) = tmpl_midi_input_map.get_function(scope) {
        let name_midi_input_map = v8::String::new(scope, "MIDIInputMap").unwrap();
        global.define_own_property(scope, name_midi_input_map.into(), ctor_midi_input_map.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_midi_output_map) = tmpl_midi_output_map.get_function(scope) {
        let name_midi_output_map = v8::String::new(scope, "MIDIOutputMap").unwrap();
        global.define_own_property(scope, name_midi_output_map.into(), ctor_midi_output_map.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_ml) = tmpl_ml.get_function(scope) {
        let name_ml = v8::String::new(scope, "ML").unwrap();
        global.define_own_property(scope, name_ml.into(), ctor_ml.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_ml_context) = tmpl_ml_context.get_function(scope) {
        let name_ml_context = v8::String::new(scope, "MLContext").unwrap();
        global.define_own_property(scope, name_ml_context.into(), ctor_ml_context.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_ml_graph) = tmpl_ml_graph.get_function(scope) {
        let name_ml_graph = v8::String::new(scope, "MLGraph").unwrap();
        global.define_own_property(scope, name_ml_graph.into(), ctor_ml_graph.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_ml_graph_builder) = tmpl_ml_graph_builder.get_function(scope) {
        let name_ml_graph_builder = v8::String::new(scope, "MLGraphBuilder").unwrap();
        global.define_own_property(scope, name_ml_graph_builder.into(), ctor_ml_graph_builder.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_ml_operand) = tmpl_ml_operand.get_function(scope) {
        let name_ml_operand = v8::String::new(scope, "MLOperand").unwrap();
        global.define_own_property(scope, name_ml_operand.into(), ctor_ml_operand.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_ml_tensor) = tmpl_ml_tensor.get_function(scope) {
        let name_ml_tensor = v8::String::new(scope, "MLTensor").unwrap();
        global.define_own_property(scope, name_ml_tensor.into(), ctor_ml_tensor.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_ndef_message) = tmpl_ndef_message.get_function(scope) {
        let name_ndef_message = v8::String::new(scope, "NDEFMessage").unwrap();
        global.define_own_property(scope, name_ndef_message.into(), ctor_ndef_message.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_ndef_record) = tmpl_ndef_record.get_function(scope) {
        let name_ndef_record = v8::String::new(scope, "NDEFRecord").unwrap();
        global.define_own_property(scope, name_ndef_record.into(), ctor_ndef_record.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_navigator_ua_data) = tmpl_navigator_ua_data.get_function(scope) {
        let name_navigator_ua_data = v8::String::new(scope, "NavigatorUAData").unwrap();
        global.define_own_property(scope, name_navigator_ua_data.into(), ctor_navigator_ua_data.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_offscreen_canvas_rendering_context2d) = tmpl_offscreen_canvas_rendering_context2d.get_function(scope) {
        let name_offscreen_canvas_rendering_context2d = v8::String::new(scope, "OffscreenCanvasRenderingContext2D").unwrap();
        global.define_own_property(scope, name_offscreen_canvas_rendering_context2d.into(), ctor_offscreen_canvas_rendering_context2d.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_origin) = tmpl_origin.get_function(scope) {
        let name_origin = v8::String::new(scope, "Origin").unwrap();
        global.define_own_property(scope, name_origin.into(), ctor_origin.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_paint_rendering_context2d) = tmpl_paint_rendering_context2d.get_function(scope) {
        let name_paint_rendering_context2d = v8::String::new(scope, "PaintRenderingContext2D").unwrap();
        global.define_own_property(scope, name_paint_rendering_context2d.into(), ctor_paint_rendering_context2d.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_paint_size) = tmpl_paint_size.get_function(scope) {
        let name_paint_size = v8::String::new(scope, "PaintSize").unwrap();
        global.define_own_property(scope, name_paint_size.into(), ctor_paint_size.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_path2d) = tmpl_path2d.get_function(scope) {
        let name_path2d = v8::String::new(scope, "Path2D").unwrap();
        global.define_own_property(scope, name_path2d.into(), ctor_path2d.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_rtc_certificate) = tmpl_rtc_certificate.get_function(scope) {
        let name_rtc_certificate = v8::String::new(scope, "RTCCertificate").unwrap();
        global.define_own_property(scope, name_rtc_certificate.into(), ctor_rtc_certificate.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_encoded_audio_frame) = tmpl_rtc_encoded_audio_frame.get_function(scope) {
        let name_rtc_encoded_audio_frame = v8::String::new(scope, "RTCEncodedAudioFrame").unwrap();
        global.define_own_property(scope, name_rtc_encoded_audio_frame.into(), ctor_rtc_encoded_audio_frame.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_encoded_video_frame) = tmpl_rtc_encoded_video_frame.get_function(scope) {
        let name_rtc_encoded_video_frame = v8::String::new(scope, "RTCEncodedVideoFrame").unwrap();
        global.define_own_property(scope, name_rtc_encoded_video_frame.into(), ctor_rtc_encoded_video_frame.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_ice_candidate) = tmpl_rtc_ice_candidate.get_function(scope) {
        let name_rtc_ice_candidate = v8::String::new(scope, "RTCIceCandidate").unwrap();
        global.define_own_property(scope, name_rtc_ice_candidate.into(), ctor_rtc_ice_candidate.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_ice_candidate_pair) = tmpl_rtc_ice_candidate_pair.get_function(scope) {
        let name_rtc_ice_candidate_pair = v8::String::new(scope, "RTCIceCandidatePair").unwrap();
        global.define_own_property(scope, name_rtc_ice_candidate_pair.into(), ctor_rtc_ice_candidate_pair.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_identity_assertion) = tmpl_rtc_identity_assertion.get_function(scope) {
        let name_rtc_identity_assertion = v8::String::new(scope, "RTCIdentityAssertion").unwrap();
        global.define_own_property(scope, name_rtc_identity_assertion.into(), ctor_rtc_identity_assertion.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_identity_provider_registrar) = tmpl_rtc_identity_provider_registrar.get_function(scope) {
        let name_rtc_identity_provider_registrar = v8::String::new(scope, "RTCIdentityProviderRegistrar").unwrap();
        global.define_own_property(scope, name_rtc_identity_provider_registrar.into(), ctor_rtc_identity_provider_registrar.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_rtp_receiver) = tmpl_rtc_rtp_receiver.get_function(scope) {
        let name_rtc_rtp_receiver = v8::String::new(scope, "RTCRtpReceiver").unwrap();
        global.define_own_property(scope, name_rtc_rtp_receiver.into(), ctor_rtc_rtp_receiver.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_rtp_s_frame_encrypter) = tmpl_rtc_rtp_s_frame_encrypter.get_function(scope) {
        let name_rtc_rtp_s_frame_encrypter = v8::String::new(scope, "RTCRtpSFrameEncrypter").unwrap();
        global.define_own_property(scope, name_rtc_rtp_s_frame_encrypter.into(), ctor_rtc_rtp_s_frame_encrypter.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_rtp_script_transform) = tmpl_rtc_rtp_script_transform.get_function(scope) {
        let name_rtc_rtp_script_transform = v8::String::new(scope, "RTCRtpScriptTransform").unwrap();
        global.define_own_property(scope, name_rtc_rtp_script_transform.into(), ctor_rtc_rtp_script_transform.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_rtp_sender) = tmpl_rtc_rtp_sender.get_function(scope) {
        let name_rtc_rtp_sender = v8::String::new(scope, "RTCRtpSender").unwrap();
        global.define_own_property(scope, name_rtc_rtp_sender.into(), ctor_rtc_rtp_sender.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_rtp_transceiver) = tmpl_rtc_rtp_transceiver.get_function(scope) {
        let name_rtc_rtp_transceiver = v8::String::new(scope, "RTCRtpTransceiver").unwrap();
        global.define_own_property(scope, name_rtc_rtp_transceiver.into(), ctor_rtc_rtp_transceiver.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_session_description) = tmpl_rtc_session_description.get_function(scope) {
        let name_rtc_session_description = v8::String::new(scope, "RTCSessionDescription").unwrap();
        global.define_own_property(scope, name_rtc_session_description.into(), ctor_rtc_session_description.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_stats_report) = tmpl_rtc_stats_report.get_function(scope) {
        let name_rtc_stats_report = v8::String::new(scope, "RTCStatsReport").unwrap();
        global.define_own_property(scope, name_rtc_stats_report.into(), ctor_rtc_stats_report.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_readable_byte_stream_controller) = tmpl_readable_byte_stream_controller.get_function(scope) {
        let name_readable_byte_stream_controller = v8::String::new(scope, "ReadableByteStreamController").unwrap();
        global.define_own_property(scope, name_readable_byte_stream_controller.into(), ctor_readable_byte_stream_controller.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_readable_stream) = tmpl_readable_stream.get_function(scope) {
        let name_readable_stream = v8::String::new(scope, "ReadableStream").unwrap();
        global.define_own_property(scope, name_readable_stream.into(), ctor_readable_stream.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_readable_stream_byob_reader) = tmpl_readable_stream_byob_reader.get_function(scope) {
        let name_readable_stream_byob_reader = v8::String::new(scope, "ReadableStreamBYOBReader").unwrap();
        global.define_own_property(scope, name_readable_stream_byob_reader.into(), ctor_readable_stream_byob_reader.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_readable_stream_byob_request) = tmpl_readable_stream_byob_request.get_function(scope) {
        let name_readable_stream_byob_request = v8::String::new(scope, "ReadableStreamBYOBRequest").unwrap();
        global.define_own_property(scope, name_readable_stream_byob_request.into(), ctor_readable_stream_byob_request.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_s_frame_encrypter_stream) = tmpl_s_frame_encrypter_stream.get_function(scope) {
        let name_s_frame_encrypter_stream = v8::String::new(scope, "SFrameEncrypterStream").unwrap();
        global.define_own_property(scope, name_s_frame_encrypter_stream.into(), ctor_s_frame_encrypter_stream.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_angle) = tmpl_svg_angle.get_function(scope) {
        let name_svg_angle = v8::String::new(scope, "SVGAngle").unwrap();
        global.define_own_property(scope, name_svg_angle.into(), ctor_svg_angle.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_animated_angle) = tmpl_svg_animated_angle.get_function(scope) {
        let name_svg_animated_angle = v8::String::new(scope, "SVGAnimatedAngle").unwrap();
        global.define_own_property(scope, name_svg_animated_angle.into(), ctor_svg_animated_angle.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_animated_boolean) = tmpl_svg_animated_boolean.get_function(scope) {
        let name_svg_animated_boolean = v8::String::new(scope, "SVGAnimatedBoolean").unwrap();
        global.define_own_property(scope, name_svg_animated_boolean.into(), ctor_svg_animated_boolean.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_animated_enumeration) = tmpl_svg_animated_enumeration.get_function(scope) {
        let name_svg_animated_enumeration = v8::String::new(scope, "SVGAnimatedEnumeration").unwrap();
        global.define_own_property(scope, name_svg_animated_enumeration.into(), ctor_svg_animated_enumeration.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_animated_integer) = tmpl_svg_animated_integer.get_function(scope) {
        let name_svg_animated_integer = v8::String::new(scope, "SVGAnimatedInteger").unwrap();
        global.define_own_property(scope, name_svg_animated_integer.into(), ctor_svg_animated_integer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_animated_length) = tmpl_svg_animated_length.get_function(scope) {
        let name_svg_animated_length = v8::String::new(scope, "SVGAnimatedLength").unwrap();
        global.define_own_property(scope, name_svg_animated_length.into(), ctor_svg_animated_length.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_animated_length_list) = tmpl_svg_animated_length_list.get_function(scope) {
        let name_svg_animated_length_list = v8::String::new(scope, "SVGAnimatedLengthList").unwrap();
        global.define_own_property(scope, name_svg_animated_length_list.into(), ctor_svg_animated_length_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_animated_number) = tmpl_svg_animated_number.get_function(scope) {
        let name_svg_animated_number = v8::String::new(scope, "SVGAnimatedNumber").unwrap();
        global.define_own_property(scope, name_svg_animated_number.into(), ctor_svg_animated_number.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_animated_number_list) = tmpl_svg_animated_number_list.get_function(scope) {
        let name_svg_animated_number_list = v8::String::new(scope, "SVGAnimatedNumberList").unwrap();
        global.define_own_property(scope, name_svg_animated_number_list.into(), ctor_svg_animated_number_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_animated_preserve_aspect_ratio) = tmpl_svg_animated_preserve_aspect_ratio.get_function(scope) {
        let name_svg_animated_preserve_aspect_ratio = v8::String::new(scope, "SVGAnimatedPreserveAspectRatio").unwrap();
        global.define_own_property(scope, name_svg_animated_preserve_aspect_ratio.into(), ctor_svg_animated_preserve_aspect_ratio.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_animated_rect) = tmpl_svg_animated_rect.get_function(scope) {
        let name_svg_animated_rect = v8::String::new(scope, "SVGAnimatedRect").unwrap();
        global.define_own_property(scope, name_svg_animated_rect.into(), ctor_svg_animated_rect.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_animated_string) = tmpl_svg_animated_string.get_function(scope) {
        let name_svg_animated_string = v8::String::new(scope, "SVGAnimatedString").unwrap();
        global.define_own_property(scope, name_svg_animated_string.into(), ctor_svg_animated_string.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_animated_transform_list) = tmpl_svg_animated_transform_list.get_function(scope) {
        let name_svg_animated_transform_list = v8::String::new(scope, "SVGAnimatedTransformList").unwrap();
        global.define_own_property(scope, name_svg_animated_transform_list.into(), ctor_svg_animated_transform_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_length) = tmpl_svg_length.get_function(scope) {
        let name_svg_length = v8::String::new(scope, "SVGLength").unwrap();
        global.define_own_property(scope, name_svg_length.into(), ctor_svg_length.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_length_list) = tmpl_svg_length_list.get_function(scope) {
        let name_svg_length_list = v8::String::new(scope, "SVGLengthList").unwrap();
        global.define_own_property(scope, name_svg_length_list.into(), ctor_svg_length_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_number) = tmpl_svg_number.get_function(scope) {
        let name_svg_number = v8::String::new(scope, "SVGNumber").unwrap();
        global.define_own_property(scope, name_svg_number.into(), ctor_svg_number.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_number_list) = tmpl_svg_number_list.get_function(scope) {
        let name_svg_number_list = v8::String::new(scope, "SVGNumberList").unwrap();
        global.define_own_property(scope, name_svg_number_list.into(), ctor_svg_number_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    // SVGPathSegment: NoInterfaceObject — skip global registration
    if let Some(ctor_svg_point_list) = tmpl_svg_point_list.get_function(scope) {
        let name_svg_point_list = v8::String::new(scope, "SVGPointList").unwrap();
        global.define_own_property(scope, name_svg_point_list.into(), ctor_svg_point_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_preserve_aspect_ratio) = tmpl_svg_preserve_aspect_ratio.get_function(scope) {
        let name_svg_preserve_aspect_ratio = v8::String::new(scope, "SVGPreserveAspectRatio").unwrap();
        global.define_own_property(scope, name_svg_preserve_aspect_ratio.into(), ctor_svg_preserve_aspect_ratio.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_string_list) = tmpl_svg_string_list.get_function(scope) {
        let name_svg_string_list = v8::String::new(scope, "SVGStringList").unwrap();
        global.define_own_property(scope, name_svg_string_list.into(), ctor_svg_string_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_transform) = tmpl_svg_transform.get_function(scope) {
        let name_svg_transform = v8::String::new(scope, "SVGTransform").unwrap();
        global.define_own_property(scope, name_svg_transform.into(), ctor_svg_transform.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_transform_list) = tmpl_svg_transform_list.get_function(scope) {
        let name_svg_transform_list = v8::String::new(scope, "SVGTransformList").unwrap();
        global.define_own_property(scope, name_svg_transform_list.into(), ctor_svg_transform_list.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_unit_types) = tmpl_svg_unit_types.get_function(scope) {
        let name_svg_unit_types = v8::String::new(scope, "SVGUnitTypes").unwrap();
        global.define_own_property(scope, name_svg_unit_types.into(), ctor_svg_unit_types.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_url_pattern) = tmpl_url_pattern.get_function(scope) {
        let name_url_pattern = v8::String::new(scope, "URLPattern").unwrap();
        global.define_own_property(scope, name_url_pattern.into(), ctor_url_pattern.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_url_search_params) = tmpl_url_search_params.get_function(scope) {
        let name_url_search_params = v8::String::new(scope, "URLSearchParams").unwrap();
        global.define_own_property(scope, name_url_search_params.into(), ctor_url_search_params.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usb_alternate_interface) = tmpl_usb_alternate_interface.get_function(scope) {
        let name_usb_alternate_interface = v8::String::new(scope, "USBAlternateInterface").unwrap();
        global.define_own_property(scope, name_usb_alternate_interface.into(), ctor_usb_alternate_interface.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usb_configuration) = tmpl_usb_configuration.get_function(scope) {
        let name_usb_configuration = v8::String::new(scope, "USBConfiguration").unwrap();
        global.define_own_property(scope, name_usb_configuration.into(), ctor_usb_configuration.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usb_device) = tmpl_usb_device.get_function(scope) {
        let name_usb_device = v8::String::new(scope, "USBDevice").unwrap();
        global.define_own_property(scope, name_usb_device.into(), ctor_usb_device.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usb_endpoint) = tmpl_usb_endpoint.get_function(scope) {
        let name_usb_endpoint = v8::String::new(scope, "USBEndpoint").unwrap();
        global.define_own_property(scope, name_usb_endpoint.into(), ctor_usb_endpoint.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usb_in_transfer_result) = tmpl_usb_in_transfer_result.get_function(scope) {
        let name_usb_in_transfer_result = v8::String::new(scope, "USBInTransferResult").unwrap();
        global.define_own_property(scope, name_usb_in_transfer_result.into(), ctor_usb_in_transfer_result.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usb_interface) = tmpl_usb_interface.get_function(scope) {
        let name_usb_interface = v8::String::new(scope, "USBInterface").unwrap();
        global.define_own_property(scope, name_usb_interface.into(), ctor_usb_interface.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usb_isochronous_in_transfer_packet) = tmpl_usb_isochronous_in_transfer_packet.get_function(scope) {
        let name_usb_isochronous_in_transfer_packet = v8::String::new(scope, "USBIsochronousInTransferPacket").unwrap();
        global.define_own_property(scope, name_usb_isochronous_in_transfer_packet.into(), ctor_usb_isochronous_in_transfer_packet.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usb_isochronous_in_transfer_result) = tmpl_usb_isochronous_in_transfer_result.get_function(scope) {
        let name_usb_isochronous_in_transfer_result = v8::String::new(scope, "USBIsochronousInTransferResult").unwrap();
        global.define_own_property(scope, name_usb_isochronous_in_transfer_result.into(), ctor_usb_isochronous_in_transfer_result.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usb_isochronous_out_transfer_packet) = tmpl_usb_isochronous_out_transfer_packet.get_function(scope) {
        let name_usb_isochronous_out_transfer_packet = v8::String::new(scope, "USBIsochronousOutTransferPacket").unwrap();
        global.define_own_property(scope, name_usb_isochronous_out_transfer_packet.into(), ctor_usb_isochronous_out_transfer_packet.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usb_isochronous_out_transfer_result) = tmpl_usb_isochronous_out_transfer_result.get_function(scope) {
        let name_usb_isochronous_out_transfer_result = v8::String::new(scope, "USBIsochronousOutTransferResult").unwrap();
        global.define_own_property(scope, name_usb_isochronous_out_transfer_result.into(), ctor_usb_isochronous_out_transfer_result.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usb_out_transfer_result) = tmpl_usb_out_transfer_result.get_function(scope) {
        let name_usb_out_transfer_result = v8::String::new(scope, "USBOutTransferResult").unwrap();
        global.define_own_property(scope, name_usb_out_transfer_result.into(), ctor_usb_out_transfer_result.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_user_activation) = tmpl_user_activation.get_function(scope) {
        let name_user_activation = v8::String::new(scope, "UserActivation").unwrap();
        global.define_own_property(scope, name_user_activation.into(), ctor_user_activation.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_vtt_region) = tmpl_vtt_region.get_function(scope) {
        let name_vtt_region = v8::String::new(scope, "VTTRegion").unwrap();
        global.define_own_property(scope, name_vtt_region.into(), ctor_vtt_region.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_wgsl_language_features) = tmpl_wgsl_language_features.get_function(scope) {
        let name_wgsl_language_features = v8::String::new(scope, "WGSLLanguageFeatures").unwrap();
        global.define_own_property(scope, name_wgsl_language_features.into(), ctor_wgsl_language_features.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_wake_lock) = tmpl_wake_lock.get_function(scope) {
        let name_wake_lock = v8::String::new(scope, "WakeLock").unwrap();
        global.define_own_property(scope, name_wake_lock.into(), ctor_wake_lock.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_gl2rendering_context) = tmpl_web_gl2rendering_context.get_function(scope) {
        let name_web_gl2rendering_context = v8::String::new(scope, "WebGL2RenderingContext").unwrap();
        global.define_own_property(scope, name_web_gl2rendering_context.into(), ctor_web_gl2rendering_context.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_gl_active_info) = tmpl_web_gl_active_info.get_function(scope) {
        let name_web_gl_active_info = v8::String::new(scope, "WebGLActiveInfo").unwrap();
        global.define_own_property(scope, name_web_gl_active_info.into(), ctor_web_gl_active_info.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_gl_object) = tmpl_web_gl_object.get_function(scope) {
        let name_web_gl_object = v8::String::new(scope, "WebGLObject").unwrap();
        global.define_own_property(scope, name_web_gl_object.into(), ctor_web_gl_object.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_gl_rendering_context) = tmpl_web_gl_rendering_context.get_function(scope) {
        let name_web_gl_rendering_context = v8::String::new(scope, "WebGLRenderingContext").unwrap();
        global.define_own_property(scope, name_web_gl_rendering_context.into(), ctor_web_gl_rendering_context.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_gl_shader_precision_format) = tmpl_web_gl_shader_precision_format.get_function(scope) {
        let name_web_gl_shader_precision_format = v8::String::new(scope, "WebGLShaderPrecisionFormat").unwrap();
        global.define_own_property(scope, name_web_gl_shader_precision_format.into(), ctor_web_gl_shader_precision_format.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_gl_uniform_location) = tmpl_web_gl_uniform_location.get_function(scope) {
        let name_web_gl_uniform_location = v8::String::new(scope, "WebGLUniformLocation").unwrap();
        global.define_own_property(scope, name_web_gl_uniform_location.into(), ctor_web_gl_uniform_location.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_xml_serializer) = tmpl_xml_serializer.get_function(scope) {
        let name_xml_serializer = v8::String::new(scope, "XMLSerializer").unwrap();
        global.define_own_property(scope, name_xml_serializer.into(), ctor_xml_serializer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_x_path_evaluator) = tmpl_x_path_evaluator.get_function(scope) {
        let name_x_path_evaluator = v8::String::new(scope, "XPathEvaluator").unwrap();
        global.define_own_property(scope, name_x_path_evaluator.into(), ctor_x_path_evaluator.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_x_path_expression) = tmpl_x_path_expression.get_function(scope) {
        let name_x_path_expression = v8::String::new(scope, "XPathExpression").unwrap();
        global.define_own_property(scope, name_x_path_expression.into(), ctor_x_path_expression.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_x_path_result) = tmpl_x_path_result.get_function(scope) {
        let name_x_path_result = v8::String::new(scope, "XPathResult").unwrap();
        global.define_own_property(scope, name_x_path_result.into(), ctor_x_path_result.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_anchor) = tmpl_xr_anchor.get_function(scope) {
        let name_xr_anchor = v8::String::new(scope, "XRAnchor").unwrap();
        global.define_own_property(scope, name_xr_anchor.into(), ctor_xr_anchor.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_anchor_set) = tmpl_xr_anchor_set.get_function(scope) {
        let name_xr_anchor_set = v8::String::new(scope, "XRAnchorSet").unwrap();
        global.define_own_property(scope, name_xr_anchor_set.into(), ctor_xr_anchor_set.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_body) = tmpl_xr_body.get_function(scope) {
        let name_xr_body = v8::String::new(scope, "XRBody").unwrap();
        global.define_own_property(scope, name_xr_body.into(), ctor_xr_body.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_camera) = tmpl_xr_camera.get_function(scope) {
        let name_xr_camera = v8::String::new(scope, "XRCamera").unwrap();
        global.define_own_property(scope, name_xr_camera.into(), ctor_xr_camera.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_depth_information) = tmpl_xr_depth_information.get_function(scope) {
        let name_xr_depth_information = v8::String::new(scope, "XRDepthInformation").unwrap();
        global.define_own_property(scope, name_xr_depth_information.into(), ctor_xr_depth_information.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_frame) = tmpl_xr_frame.get_function(scope) {
        let name_xr_frame = v8::String::new(scope, "XRFrame").unwrap();
        global.define_own_property(scope, name_xr_frame.into(), ctor_xr_frame.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_hand) = tmpl_xr_hand.get_function(scope) {
        let name_xr_hand = v8::String::new(scope, "XRHand").unwrap();
        global.define_own_property(scope, name_xr_hand.into(), ctor_xr_hand.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_hit_test_result) = tmpl_xr_hit_test_result.get_function(scope) {
        let name_xr_hit_test_result = v8::String::new(scope, "XRHitTestResult").unwrap();
        global.define_own_property(scope, name_xr_hit_test_result.into(), ctor_xr_hit_test_result.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_hit_test_source) = tmpl_xr_hit_test_source.get_function(scope) {
        let name_xr_hit_test_source = v8::String::new(scope, "XRHitTestSource").unwrap();
        global.define_own_property(scope, name_xr_hit_test_source.into(), ctor_xr_hit_test_source.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_input_source) = tmpl_xr_input_source.get_function(scope) {
        let name_xr_input_source = v8::String::new(scope, "XRInputSource").unwrap();
        global.define_own_property(scope, name_xr_input_source.into(), ctor_xr_input_source.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_input_source_array) = tmpl_xr_input_source_array.get_function(scope) {
        let name_xr_input_source_array = v8::String::new(scope, "XRInputSourceArray").unwrap();
        global.define_own_property(scope, name_xr_input_source_array.into(), ctor_xr_input_source_array.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_light_estimate) = tmpl_xr_light_estimate.get_function(scope) {
        let name_xr_light_estimate = v8::String::new(scope, "XRLightEstimate").unwrap();
        global.define_own_property(scope, name_xr_light_estimate.into(), ctor_xr_light_estimate.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_media_binding) = tmpl_xr_media_binding.get_function(scope) {
        let name_xr_media_binding = v8::String::new(scope, "XRMediaBinding").unwrap();
        global.define_own_property(scope, name_xr_media_binding.into(), ctor_xr_media_binding.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_mesh) = tmpl_xr_mesh.get_function(scope) {
        let name_xr_mesh = v8::String::new(scope, "XRMesh").unwrap();
        global.define_own_property(scope, name_xr_mesh.into(), ctor_xr_mesh.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_mesh_set) = tmpl_xr_mesh_set.get_function(scope) {
        let name_xr_mesh_set = v8::String::new(scope, "XRMeshSet").unwrap();
        global.define_own_property(scope, name_xr_mesh_set.into(), ctor_xr_mesh_set.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_plane) = tmpl_xr_plane.get_function(scope) {
        let name_xr_plane = v8::String::new(scope, "XRPlane").unwrap();
        global.define_own_property(scope, name_xr_plane.into(), ctor_xr_plane.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_plane_set) = tmpl_xr_plane_set.get_function(scope) {
        let name_xr_plane_set = v8::String::new(scope, "XRPlaneSet").unwrap();
        global.define_own_property(scope, name_xr_plane_set.into(), ctor_xr_plane_set.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_pose) = tmpl_xr_pose.get_function(scope) {
        let name_xr_pose = v8::String::new(scope, "XRPose").unwrap();
        global.define_own_property(scope, name_xr_pose.into(), ctor_xr_pose.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_ray) = tmpl_xr_ray.get_function(scope) {
        let name_xr_ray = v8::String::new(scope, "XRRay").unwrap();
        global.define_own_property(scope, name_xr_ray.into(), ctor_xr_ray.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_render_state) = tmpl_xr_render_state.get_function(scope) {
        let name_xr_render_state = v8::String::new(scope, "XRRenderState").unwrap();
        global.define_own_property(scope, name_xr_render_state.into(), ctor_xr_render_state.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_rigid_transform) = tmpl_xr_rigid_transform.get_function(scope) {
        let name_xr_rigid_transform = v8::String::new(scope, "XRRigidTransform").unwrap();
        global.define_own_property(scope, name_xr_rigid_transform.into(), ctor_xr_rigid_transform.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_sub_image) = tmpl_xr_sub_image.get_function(scope) {
        let name_xr_sub_image = v8::String::new(scope, "XRSubImage").unwrap();
        global.define_own_property(scope, name_xr_sub_image.into(), ctor_xr_sub_image.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_transient_input_hit_test_result) = tmpl_xr_transient_input_hit_test_result.get_function(scope) {
        let name_xr_transient_input_hit_test_result = v8::String::new(scope, "XRTransientInputHitTestResult").unwrap();
        global.define_own_property(scope, name_xr_transient_input_hit_test_result.into(), ctor_xr_transient_input_hit_test_result.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_transient_input_hit_test_source) = tmpl_xr_transient_input_hit_test_source.get_function(scope) {
        let name_xr_transient_input_hit_test_source = v8::String::new(scope, "XRTransientInputHitTestSource").unwrap();
        global.define_own_property(scope, name_xr_transient_input_hit_test_source.into(), ctor_xr_transient_input_hit_test_source.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_view) = tmpl_xr_view.get_function(scope) {
        let name_xr_view = v8::String::new(scope, "XRView").unwrap();
        global.define_own_property(scope, name_xr_view.into(), ctor_xr_view.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_viewport) = tmpl_xr_viewport.get_function(scope) {
        let name_xr_viewport = v8::String::new(scope, "XRViewport").unwrap();
        global.define_own_property(scope, name_xr_viewport.into(), ctor_xr_viewport.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_web_gl_binding) = tmpl_xr_web_gl_binding.get_function(scope) {
        let name_xr_web_gl_binding = v8::String::new(scope, "XRWebGLBinding").unwrap();
        global.define_own_property(scope, name_xr_web_gl_binding.into(), ctor_xr_web_gl_binding.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xslt_processor) = tmpl_xslt_processor.get_function(scope) {
        let name_xslt_processor = v8::String::new(scope, "XSLTProcessor").unwrap();
        global.define_own_property(scope, name_xslt_processor.into(), ctor_xslt_processor.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_webkit_idb_key_range) = tmpl_webkit_idb_key_range.get_function(scope) {
        let name_webkit_idb_key_range = v8::String::new(scope, "webkitIDBKeyRange").unwrap();
        global.define_own_property(scope, name_webkit_idb_key_range.into(), ctor_webkit_idb_key_range.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_webkit_rtc_peer_connection) = tmpl_webkit_rtc_peer_connection.get_function(scope) {
        let name_webkit_rtc_peer_connection = v8::String::new(scope, "webkitRTCPeerConnection").unwrap();
        global.define_own_property(scope, name_webkit_rtc_peer_connection.into(), ctor_webkit_rtc_peer_connection.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_css_parser_at_rule) = tmpl_css_parser_at_rule.get_function(scope) {
        let name_css_parser_at_rule = v8::String::new(scope, "CSSParserAtRule").unwrap();
        global.define_own_property(scope, name_css_parser_at_rule.into(), ctor_css_parser_at_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_parser_declaration) = tmpl_css_parser_declaration.get_function(scope) {
        let name_css_parser_declaration = v8::String::new(scope, "CSSParserDeclaration").unwrap();
        global.define_own_property(scope, name_css_parser_declaration.into(), ctor_css_parser_declaration.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_parser_qualified_rule) = tmpl_css_parser_qualified_rule.get_function(scope) {
        let name_css_parser_qualified_rule = v8::String::new(scope, "CSSParserQualifiedRule").unwrap();
        global.define_own_property(scope, name_css_parser_qualified_rule.into(), ctor_css_parser_qualified_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_parser_block) = tmpl_css_parser_block.get_function(scope) {
        let name_css_parser_block = v8::String::new(scope, "CSSParserBlock").unwrap();
        global.define_own_property(scope, name_css_parser_block.into(), ctor_css_parser_block.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_parser_function) = tmpl_css_parser_function.get_function(scope) {
        let name_css_parser_function = v8::String::new(scope, "CSSParserFunction").unwrap();
        global.define_own_property(scope, name_css_parser_function.into(), ctor_css_parser_function.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_apply_statement_rule) = tmpl_css_apply_statement_rule.get_function(scope) {
        let name_css_apply_statement_rule = v8::String::new(scope, "CSSApplyStatementRule").unwrap();
        global.define_own_property(scope, name_css_apply_statement_rule.into(), ctor_css_apply_statement_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_color_profile_rule) = tmpl_css_color_profile_rule.get_function(scope) {
        let name_css_color_profile_rule = v8::String::new(scope, "CSSColorProfileRule").unwrap();
        global.define_own_property(scope, name_css_color_profile_rule.into(), ctor_css_color_profile_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_contents_statement_rule) = tmpl_css_contents_statement_rule.get_function(scope) {
        let name_css_contents_statement_rule = v8::String::new(scope, "CSSContentsStatementRule").unwrap();
        global.define_own_property(scope, name_css_contents_statement_rule.into(), ctor_css_contents_statement_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_counter_style_rule) = tmpl_css_counter_style_rule.get_function(scope) {
        let name_css_counter_style_rule = v8::String::new(scope, "CSSCounterStyleRule").unwrap();
        global.define_own_property(scope, name_css_counter_style_rule.into(), ctor_css_counter_style_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_custom_media_rule) = tmpl_css_custom_media_rule.get_function(scope) {
        let name_css_custom_media_rule = v8::String::new(scope, "CSSCustomMediaRule").unwrap();
        global.define_own_property(scope, name_css_custom_media_rule.into(), ctor_css_custom_media_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_font_face_rule) = tmpl_css_font_face_rule.get_function(scope) {
        let name_css_font_face_rule = v8::String::new(scope, "CSSFontFaceRule").unwrap();
        global.define_own_property(scope, name_css_font_face_rule.into(), ctor_css_font_face_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_font_feature_values_rule) = tmpl_css_font_feature_values_rule.get_function(scope) {
        let name_css_font_feature_values_rule = v8::String::new(scope, "CSSFontFeatureValuesRule").unwrap();
        global.define_own_property(scope, name_css_font_feature_values_rule.into(), ctor_css_font_feature_values_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_font_palette_values_rule) = tmpl_css_font_palette_values_rule.get_function(scope) {
        let name_css_font_palette_values_rule = v8::String::new(scope, "CSSFontPaletteValuesRule").unwrap();
        global.define_own_property(scope, name_css_font_palette_values_rule.into(), ctor_css_font_palette_values_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_function_declarations) = tmpl_css_function_declarations.get_function(scope) {
        let name_css_function_declarations = v8::String::new(scope, "CSSFunctionDeclarations").unwrap();
        global.define_own_property(scope, name_css_function_declarations.into(), ctor_css_function_declarations.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_grouping_rule) = tmpl_css_grouping_rule.get_function(scope) {
        let name_css_grouping_rule = v8::String::new(scope, "CSSGroupingRule").unwrap();
        global.define_own_property(scope, name_css_grouping_rule.into(), ctor_css_grouping_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_import_rule) = tmpl_css_import_rule.get_function(scope) {
        let name_css_import_rule = v8::String::new(scope, "CSSImportRule").unwrap();
        global.define_own_property(scope, name_css_import_rule.into(), ctor_css_import_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_keyframe_rule) = tmpl_css_keyframe_rule.get_function(scope) {
        let name_css_keyframe_rule = v8::String::new(scope, "CSSKeyframeRule").unwrap();
        global.define_own_property(scope, name_css_keyframe_rule.into(), ctor_css_keyframe_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_keyframes_rule) = tmpl_css_keyframes_rule.get_function(scope) {
        let name_css_keyframes_rule = v8::String::new(scope, "CSSKeyframesRule").unwrap();
        global.define_own_property(scope, name_css_keyframes_rule.into(), ctor_css_keyframes_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_layer_statement_rule) = tmpl_css_layer_statement_rule.get_function(scope) {
        let name_css_layer_statement_rule = v8::String::new(scope, "CSSLayerStatementRule").unwrap();
        global.define_own_property(scope, name_css_layer_statement_rule.into(), ctor_css_layer_statement_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_margin_rule) = tmpl_css_margin_rule.get_function(scope) {
        let name_css_margin_rule = v8::String::new(scope, "CSSMarginRule").unwrap();
        global.define_own_property(scope, name_css_margin_rule.into(), ctor_css_margin_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_namespace_rule) = tmpl_css_namespace_rule.get_function(scope) {
        let name_css_namespace_rule = v8::String::new(scope, "CSSNamespaceRule").unwrap();
        global.define_own_property(scope, name_css_namespace_rule.into(), ctor_css_namespace_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_nested_declarations) = tmpl_css_nested_declarations.get_function(scope) {
        let name_css_nested_declarations = v8::String::new(scope, "CSSNestedDeclarations").unwrap();
        global.define_own_property(scope, name_css_nested_declarations.into(), ctor_css_nested_declarations.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_position_try_rule) = tmpl_css_position_try_rule.get_function(scope) {
        let name_css_position_try_rule = v8::String::new(scope, "CSSPositionTryRule").unwrap();
        global.define_own_property(scope, name_css_position_try_rule.into(), ctor_css_position_try_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_property_rule) = tmpl_css_property_rule.get_function(scope) {
        let name_css_property_rule = v8::String::new(scope, "CSSPropertyRule").unwrap();
        global.define_own_property(scope, name_css_property_rule.into(), ctor_css_property_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_view_transition_rule) = tmpl_css_view_transition_rule.get_function(scope) {
        let name_css_view_transition_rule = v8::String::new(scope, "CSSViewTransitionRule").unwrap();
        global.define_own_property(scope, name_css_view_transition_rule.into(), ctor_css_view_transition_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_font_face_descriptors) = tmpl_css_font_face_descriptors.get_function(scope) {
        let name_css_font_face_descriptors = v8::String::new(scope, "CSSFontFaceDescriptors").unwrap();
        global.define_own_property(scope, name_css_font_face_descriptors.into(), ctor_css_font_face_descriptors.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_function_descriptors) = tmpl_css_function_descriptors.get_function(scope) {
        let name_css_function_descriptors = v8::String::new(scope, "CSSFunctionDescriptors").unwrap();
        global.define_own_property(scope, name_css_function_descriptors.into(), ctor_css_function_descriptors.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_page_descriptors) = tmpl_css_page_descriptors.get_function(scope) {
        let name_css_page_descriptors = v8::String::new(scope, "CSSPageDescriptors").unwrap();
        global.define_own_property(scope, name_css_page_descriptors.into(), ctor_css_page_descriptors.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_position_try_descriptors) = tmpl_css_position_try_descriptors.get_function(scope) {
        let name_css_position_try_descriptors = v8::String::new(scope, "CSSPositionTryDescriptors").unwrap();
        global.define_own_property(scope, name_css_position_try_descriptors.into(), ctor_css_position_try_descriptors.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_style_properties) = tmpl_css_style_properties.get_function(scope) {
        let name_css_style_properties = v8::String::new(scope, "CSSStyleProperties").unwrap();
        global.define_own_property(scope, name_css_style_properties.into(), ctor_css_style_properties.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_color_value) = tmpl_css_color_value.get_function(scope) {
        let name_css_color_value = v8::String::new(scope, "CSSColorValue").unwrap();
        global.define_own_property(scope, name_css_color_value.into(), ctor_css_color_value.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_image_value) = tmpl_css_image_value.get_function(scope) {
        let name_css_image_value = v8::String::new(scope, "CSSImageValue").unwrap();
        global.define_own_property(scope, name_css_image_value.into(), ctor_css_image_value.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_keyword_value) = tmpl_css_keyword_value.get_function(scope) {
        let name_css_keyword_value = v8::String::new(scope, "CSSKeywordValue").unwrap();
        global.define_own_property(scope, name_css_keyword_value.into(), ctor_css_keyword_value.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_numeric_value) = tmpl_css_numeric_value.get_function(scope) {
        let name_css_numeric_value = v8::String::new(scope, "CSSNumericValue").unwrap();
        global.define_own_property(scope, name_css_numeric_value.into(), ctor_css_numeric_value.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_transform_value) = tmpl_css_transform_value.get_function(scope) {
        let name_css_transform_value = v8::String::new(scope, "CSSTransformValue").unwrap();
        global.define_own_property(scope, name_css_transform_value.into(), ctor_css_transform_value.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_unparsed_value) = tmpl_css_unparsed_value.get_function(scope) {
        let name_css_unparsed_value = v8::String::new(scope, "CSSUnparsedValue").unwrap();
        global.define_own_property(scope, name_css_unparsed_value.into(), ctor_css_unparsed_value.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_matrix_component) = tmpl_css_matrix_component.get_function(scope) {
        let name_css_matrix_component = v8::String::new(scope, "CSSMatrixComponent").unwrap();
        global.define_own_property(scope, name_css_matrix_component.into(), ctor_css_matrix_component.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_perspective) = tmpl_css_perspective.get_function(scope) {
        let name_css_perspective = v8::String::new(scope, "CSSPerspective").unwrap();
        global.define_own_property(scope, name_css_perspective.into(), ctor_css_perspective.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_rotate) = tmpl_css_rotate.get_function(scope) {
        let name_css_rotate = v8::String::new(scope, "CSSRotate").unwrap();
        global.define_own_property(scope, name_css_rotate.into(), ctor_css_rotate.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_scale) = tmpl_css_scale.get_function(scope) {
        let name_css_scale = v8::String::new(scope, "CSSScale").unwrap();
        global.define_own_property(scope, name_css_scale.into(), ctor_css_scale.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_skew) = tmpl_css_skew.get_function(scope) {
        let name_css_skew = v8::String::new(scope, "CSSSkew").unwrap();
        global.define_own_property(scope, name_css_skew.into(), ctor_css_skew.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_skew_x) = tmpl_css_skew_x.get_function(scope) {
        let name_css_skew_x = v8::String::new(scope, "CSSSkewX").unwrap();
        global.define_own_property(scope, name_css_skew_x.into(), ctor_css_skew_x.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_skew_y) = tmpl_css_skew_y.get_function(scope) {
        let name_css_skew_y = v8::String::new(scope, "CSSSkewY").unwrap();
        global.define_own_property(scope, name_css_skew_y.into(), ctor_css_skew_y.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_translate) = tmpl_css_translate.get_function(scope) {
        let name_css_translate = v8::String::new(scope, "CSSTranslate").unwrap();
        global.define_own_property(scope, name_css_translate.into(), ctor_css_translate.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_otp_credential) = tmpl_otp_credential.get_function(scope) {
        let name_otp_credential = v8::String::new(scope, "OTPCredential").unwrap();
        global.define_own_property(scope, name_otp_credential.into(), ctor_otp_credential.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_password_credential) = tmpl_password_credential.get_function(scope) {
        let name_password_credential = v8::String::new(scope, "PasswordCredential").unwrap();
        global.define_own_property(scope, name_password_credential.into(), ctor_password_credential.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_public_key_credential) = tmpl_public_key_credential.get_function(scope) {
        let name_public_key_credential = v8::String::new(scope, "PublicKeyCredential").unwrap();
        global.define_own_property(scope, name_public_key_credential.into(), ctor_public_key_credential.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_pipeline_error) = tmpl_gpu_pipeline_error.get_function(scope) {
        let name_gpu_pipeline_error = v8::String::new(scope, "GPUPipelineError").unwrap();
        global.define_own_property(scope, name_gpu_pipeline_error.into(), ctor_gpu_pipeline_error.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_rtc_error) = tmpl_rtc_error.get_function(scope) {
        let name_rtc_error = v8::String::new(scope, "RTCError").unwrap();
        global.define_own_property(scope, name_rtc_error.into(), ctor_rtc_error.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_transport_error) = tmpl_web_transport_error.get_function(scope) {
        let name_web_transport_error = v8::String::new(scope, "WebTransportError").unwrap();
        global.define_own_property(scope, name_web_transport_error.into(), ctor_web_transport_error.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_dom_matrix) = tmpl_dom_matrix.get_function(scope) {
        let name_dom_matrix = v8::String::new(scope, "DOMMatrix").unwrap();
        global.define_own_property(scope, name_dom_matrix.into(), ctor_dom_matrix.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_dom_point) = tmpl_dom_point.get_function(scope) {
        let name_dom_point = v8::String::new(scope, "DOMPoint").unwrap();
        global.define_own_property(scope, name_dom_point.into(), ctor_dom_point.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_dom_rect) = tmpl_dom_rect.get_function(scope) {
        let name_dom_rect = v8::String::new(scope, "DOMRect").unwrap();
        global.define_own_property(scope, name_dom_rect.into(), ctor_dom_rect.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_gpu_uncaptured_error_event) = tmpl_gpu_uncaptured_error_event.get_function(scope) {
        let name_gpu_uncaptured_error_event = v8::String::new(scope, "GPUUncapturedErrorEvent").unwrap();
        global.define_own_property(scope, name_gpu_uncaptured_error_event.into(), ctor_gpu_uncaptured_error_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gamepad_event) = tmpl_gamepad_event.get_function(scope) {
        let name_gamepad_event = v8::String::new(scope, "GamepadEvent").unwrap();
        global.define_own_property(scope, name_gamepad_event.into(), ctor_gamepad_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_hid_connection_event) = tmpl_hid_connection_event.get_function(scope) {
        let name_hid_connection_event = v8::String::new(scope, "HIDConnectionEvent").unwrap();
        global.define_own_property(scope, name_hid_connection_event.into(), ctor_hid_connection_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_hid_input_report_event) = tmpl_hid_input_report_event.get_function(scope) {
        let name_hid_input_report_event = v8::String::new(scope, "HIDInputReportEvent").unwrap();
        global.define_own_property(scope, name_hid_input_report_event.into(), ctor_hid_input_report_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_hash_change_event) = tmpl_hash_change_event.get_function(scope) {
        let name_hash_change_event = v8::String::new(scope, "HashChangeEvent").unwrap();
        global.define_own_property(scope, name_hash_change_event.into(), ctor_hash_change_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idb_version_change_event) = tmpl_idb_version_change_event.get_function(scope) {
        let name_idb_version_change_event = v8::String::new(scope, "IDBVersionChangeEvent").unwrap();
        global.define_own_property(scope, name_idb_version_change_event.into(), ctor_idb_version_change_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_key_frame_request_event) = tmpl_key_frame_request_event.get_function(scope) {
        let name_key_frame_request_event = v8::String::new(scope, "KeyFrameRequestEvent").unwrap();
        global.define_own_property(scope, name_key_frame_request_event.into(), ctor_key_frame_request_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_midi_connection_event) = tmpl_midi_connection_event.get_function(scope) {
        let name_midi_connection_event = v8::String::new(scope, "MIDIConnectionEvent").unwrap();
        global.define_own_property(scope, name_midi_connection_event.into(), ctor_midi_connection_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_midi_message_event) = tmpl_midi_message_event.get_function(scope) {
        let name_midi_message_event = v8::String::new(scope, "MIDIMessageEvent").unwrap();
        global.define_own_property(scope, name_midi_message_event.into(), ctor_midi_message_event.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_ndef_reading_event) = tmpl_ndef_reading_event.get_function(scope) {
        let name_ndef_reading_event = v8::String::new(scope, "NDEFReadingEvent").unwrap();
        global.define_own_property(scope, name_ndef_reading_event.into(), ctor_ndef_reading_event.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_rtcdtmf_tone_change_event) = tmpl_rtcdtmf_tone_change_event.get_function(scope) {
        let name_rtcdtmf_tone_change_event = v8::String::new(scope, "RTCDTMFToneChangeEvent").unwrap();
        global.define_own_property(scope, name_rtcdtmf_tone_change_event.into(), ctor_rtcdtmf_tone_change_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_data_channel_event) = tmpl_rtc_data_channel_event.get_function(scope) {
        let name_rtc_data_channel_event = v8::String::new(scope, "RTCDataChannelEvent").unwrap();
        global.define_own_property(scope, name_rtc_data_channel_event.into(), ctor_rtc_data_channel_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_error_event) = tmpl_rtc_error_event.get_function(scope) {
        let name_rtc_error_event = v8::String::new(scope, "RTCErrorEvent").unwrap();
        global.define_own_property(scope, name_rtc_error_event.into(), ctor_rtc_error_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_peer_connection_ice_error_event) = tmpl_rtc_peer_connection_ice_error_event.get_function(scope) {
        let name_rtc_peer_connection_ice_error_event = v8::String::new(scope, "RTCPeerConnectionIceErrorEvent").unwrap();
        global.define_own_property(scope, name_rtc_peer_connection_ice_error_event.into(), ctor_rtc_peer_connection_ice_error_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_peer_connection_ice_event) = tmpl_rtc_peer_connection_ice_event.get_function(scope) {
        let name_rtc_peer_connection_ice_event = v8::String::new(scope, "RTCPeerConnectionIceEvent").unwrap();
        global.define_own_property(scope, name_rtc_peer_connection_ice_event.into(), ctor_rtc_peer_connection_ice_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_track_event) = tmpl_rtc_track_event.get_function(scope) {
        let name_rtc_track_event = v8::String::new(scope, "RTCTrackEvent").unwrap();
        global.define_own_property(scope, name_rtc_track_event.into(), ctor_rtc_track_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_transform_event) = tmpl_rtc_transform_event.get_function(scope) {
        let name_rtc_transform_event = v8::String::new(scope, "RTCTransformEvent").unwrap();
        global.define_own_property(scope, name_rtc_transform_event.into(), ctor_rtc_transform_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_s_frame_transform_error_event) = tmpl_s_frame_transform_error_event.get_function(scope) {
        let name_s_frame_transform_error_event = v8::String::new(scope, "SFrameTransformErrorEvent").unwrap();
        global.define_own_property(scope, name_s_frame_transform_error_event.into(), ctor_s_frame_transform_error_event.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_ui_event) = tmpl_ui_event.get_function(scope) {
        let name_ui_event = v8::String::new(scope, "UIEvent").unwrap();
        global.define_own_property(scope, name_ui_event.into(), ctor_ui_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usb_connection_event) = tmpl_usb_connection_event.get_function(scope) {
        let name_usb_connection_event = v8::String::new(scope, "USBConnectionEvent").unwrap();
        global.define_own_property(scope, name_usb_connection_event.into(), ctor_usb_connection_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_value_event) = tmpl_value_event.get_function(scope) {
        let name_value_event = v8::String::new(scope, "ValueEvent").unwrap();
        global.define_own_property(scope, name_value_event.into(), ctor_value_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_gl_context_event) = tmpl_web_gl_context_event.get_function(scope) {
        let name_web_gl_context_event = v8::String::new(scope, "WebGLContextEvent").unwrap();
        global.define_own_property(scope, name_web_gl_context_event.into(), ctor_web_gl_context_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_window_controls_overlay_geometry_change_event) = tmpl_window_controls_overlay_geometry_change_event.get_function(scope) {
        let name_window_controls_overlay_geometry_change_event = v8::String::new(scope, "WindowControlsOverlayGeometryChangeEvent").unwrap();
        global.define_own_property(scope, name_window_controls_overlay_geometry_change_event.into(), ctor_window_controls_overlay_geometry_change_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_input_source_event) = tmpl_xr_input_source_event.get_function(scope) {
        let name_xr_input_source_event = v8::String::new(scope, "XRInputSourceEvent").unwrap();
        global.define_own_property(scope, name_xr_input_source_event.into(), ctor_xr_input_source_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_input_sources_change_event) = tmpl_xr_input_sources_change_event.get_function(scope) {
        let name_xr_input_sources_change_event = v8::String::new(scope, "XRInputSourcesChangeEvent").unwrap();
        global.define_own_property(scope, name_xr_input_sources_change_event.into(), ctor_xr_input_sources_change_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_layer_event) = tmpl_xr_layer_event.get_function(scope) {
        let name_xr_layer_event = v8::String::new(scope, "XRLayerEvent").unwrap();
        global.define_own_property(scope, name_xr_layer_event.into(), ctor_xr_layer_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_reference_space_event) = tmpl_xr_reference_space_event.get_function(scope) {
        let name_xr_reference_space_event = v8::String::new(scope, "XRReferenceSpaceEvent").unwrap();
        global.define_own_property(scope, name_xr_reference_space_event.into(), ctor_xr_reference_space_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_session_event) = tmpl_xr_session_event.get_function(scope) {
        let name_xr_session_event = v8::String::new(scope, "XRSessionEvent").unwrap();
        global.define_own_property(scope, name_xr_session_event.into(), ctor_xr_session_event.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_visibility_mask_change_event) = tmpl_xr_visibility_mask_change_event.get_function(scope) {
        let name_xr_visibility_mask_change_event = v8::String::new(scope, "XRVisibilityMaskChangeEvent").unwrap();
        global.define_own_property(scope, name_xr_visibility_mask_change_event.into(), ctor_xr_visibility_mask_change_event.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_bluetooth_remote_gatt_characteristic) = tmpl_bluetooth_remote_gatt_characteristic.get_function(scope) {
        let name_bluetooth_remote_gatt_characteristic = v8::String::new(scope, "BluetoothRemoteGATTCharacteristic").unwrap();
        global.define_own_property(scope, name_bluetooth_remote_gatt_characteristic.into(), ctor_bluetooth_remote_gatt_characteristic.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_remote_gatt_service) = tmpl_bluetooth_remote_gatt_service.get_function(scope) {
        let name_bluetooth_remote_gatt_service = v8::String::new(scope, "BluetoothRemoteGATTService").unwrap();
        global.define_own_property(scope, name_bluetooth_remote_gatt_service.into(), ctor_bluetooth_remote_gatt_service.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_gpu_device) = tmpl_gpu_device.get_function(scope) {
        let name_gpu_device = v8::String::new(scope, "GPUDevice").unwrap();
        global.define_own_property(scope, name_gpu_device.into(), ctor_gpu_device.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_hid) = tmpl_hid.get_function(scope) {
        let name_hid = v8::String::new(scope, "HID").unwrap();
        global.define_own_property(scope, name_hid.into(), ctor_hid.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_hid_device) = tmpl_hid_device.get_function(scope) {
        let name_hid_device = v8::String::new(scope, "HIDDevice").unwrap();
        global.define_own_property(scope, name_hid_device.into(), ctor_hid_device.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idb_database) = tmpl_idb_database.get_function(scope) {
        let name_idb_database = v8::String::new(scope, "IDBDatabase").unwrap();
        global.define_own_property(scope, name_idb_database.into(), ctor_idb_database.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idb_request) = tmpl_idb_request.get_function(scope) {
        let name_idb_request = v8::String::new(scope, "IDBRequest").unwrap();
        global.define_own_property(scope, name_idb_request.into(), ctor_idb_request.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idb_transaction) = tmpl_idb_transaction.get_function(scope) {
        let name_idb_transaction = v8::String::new(scope, "IDBTransaction").unwrap();
        global.define_own_property(scope, name_idb_transaction.into(), ctor_idb_transaction.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_midi_access) = tmpl_midi_access.get_function(scope) {
        let name_midi_access = v8::String::new(scope, "MIDIAccess").unwrap();
        global.define_own_property(scope, name_midi_access.into(), ctor_midi_access.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_midi_port) = tmpl_midi_port.get_function(scope) {
        let name_midi_port = v8::String::new(scope, "MIDIPort").unwrap();
        global.define_own_property(scope, name_midi_port.into(), ctor_midi_port.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_ndef_reader) = tmpl_ndef_reader.get_function(scope) {
        let name_ndef_reader = v8::String::new(scope, "NDEFReader").unwrap();
        global.define_own_property(scope, name_ndef_reader.into(), ctor_ndef_reader.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_rtcdtmf_sender) = tmpl_rtcdtmf_sender.get_function(scope) {
        let name_rtcdtmf_sender = v8::String::new(scope, "RTCDTMFSender").unwrap();
        global.define_own_property(scope, name_rtcdtmf_sender.into(), ctor_rtcdtmf_sender.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_data_channel) = tmpl_rtc_data_channel.get_function(scope) {
        let name_rtc_data_channel = v8::String::new(scope, "RTCDataChannel").unwrap();
        global.define_own_property(scope, name_rtc_data_channel.into(), ctor_rtc_data_channel.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_dtls_transport) = tmpl_rtc_dtls_transport.get_function(scope) {
        let name_rtc_dtls_transport = v8::String::new(scope, "RTCDtlsTransport").unwrap();
        global.define_own_property(scope, name_rtc_dtls_transport.into(), ctor_rtc_dtls_transport.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_ice_transport) = tmpl_rtc_ice_transport.get_function(scope) {
        let name_rtc_ice_transport = v8::String::new(scope, "RTCIceTransport").unwrap();
        global.define_own_property(scope, name_rtc_ice_transport.into(), ctor_rtc_ice_transport.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_peer_connection) = tmpl_rtc_peer_connection.get_function(scope) {
        let name_rtc_peer_connection = v8::String::new(scope, "RTCPeerConnection").unwrap();
        global.define_own_property(scope, name_rtc_peer_connection.into(), ctor_rtc_peer_connection.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_rtp_s_frame_decrypter) = tmpl_rtc_rtp_s_frame_decrypter.get_function(scope) {
        let name_rtc_rtp_s_frame_decrypter = v8::String::new(scope, "RTCRtpSFrameDecrypter").unwrap();
        global.define_own_property(scope, name_rtc_rtp_s_frame_decrypter.into(), ctor_rtc_rtp_s_frame_decrypter.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_rtp_script_transformer) = tmpl_rtc_rtp_script_transformer.get_function(scope) {
        let name_rtc_rtp_script_transformer = v8::String::new(scope, "RTCRtpScriptTransformer").unwrap();
        global.define_own_property(scope, name_rtc_rtp_script_transformer.into(), ctor_rtc_rtp_script_transformer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_sctp_transport) = tmpl_rtc_sctp_transport.get_function(scope) {
        let name_rtc_sctp_transport = v8::String::new(scope, "RTCSctpTransport").unwrap();
        global.define_own_property(scope, name_rtc_sctp_transport.into(), ctor_rtc_sctp_transport.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_remote_playback) = tmpl_remote_playback.get_function(scope) {
        let name_remote_playback = v8::String::new(scope, "RemotePlayback").unwrap();
        global.define_own_property(scope, name_remote_playback.into(), ctor_remote_playback.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_s_frame_decrypter_stream) = tmpl_s_frame_decrypter_stream.get_function(scope) {
        let name_s_frame_decrypter_stream = v8::String::new(scope, "SFrameDecrypterStream").unwrap();
        global.define_own_property(scope, name_s_frame_decrypter_stream.into(), ctor_s_frame_decrypter_stream.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_xml_http_request_event_target) = tmpl_xml_http_request_event_target.get_function(scope) {
        let name_xml_http_request_event_target = v8::String::new(scope, "XMLHttpRequestEventTarget").unwrap();
        global.define_own_property(scope, name_xml_http_request_event_target.into(), ctor_xml_http_request_event_target.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_layer) = tmpl_xr_layer.get_function(scope) {
        let name_xr_layer = v8::String::new(scope, "XRLayer").unwrap();
        global.define_own_property(scope, name_xr_layer.into(), ctor_xr_layer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_light_probe) = tmpl_xr_light_probe.get_function(scope) {
        let name_xr_light_probe = v8::String::new(scope, "XRLightProbe").unwrap();
        global.define_own_property(scope, name_xr_light_probe.into(), ctor_xr_light_probe.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_session) = tmpl_xr_session.get_function(scope) {
        let name_xr_session = v8::String::new(scope, "XRSession").unwrap();
        global.define_own_property(scope, name_xr_session.into(), ctor_xr_session.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_space) = tmpl_xr_space.get_function(scope) {
        let name_xr_space = v8::String::new(scope, "XRSpace").unwrap();
        global.define_own_property(scope, name_xr_space.into(), ctor_xr_space.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_system) = tmpl_xr_system.get_function(scope) {
        let name_xr_system = v8::String::new(scope, "XRSystem").unwrap();
        global.define_own_property(scope, name_xr_system.into(), ctor_xr_system.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_gpu_internal_error) = tmpl_gpu_internal_error.get_function(scope) {
        let name_gpu_internal_error = v8::String::new(scope, "GPUInternalError").unwrap();
        global.define_own_property(scope, name_gpu_internal_error.into(), ctor_gpu_internal_error.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_out_of_memory_error) = tmpl_gpu_out_of_memory_error.get_function(scope) {
        let name_gpu_out_of_memory_error = v8::String::new(scope, "GPUOutOfMemoryError").unwrap();
        global.define_own_property(scope, name_gpu_out_of_memory_error.into(), ctor_gpu_out_of_memory_error.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_gpu_validation_error) = tmpl_gpu_validation_error.get_function(scope) {
        let name_gpu_validation_error = v8::String::new(scope, "GPUValidationError").unwrap();
        global.define_own_property(scope, name_gpu_validation_error.into(), ctor_gpu_validation_error.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_sequence_effect) = tmpl_sequence_effect.get_function(scope) {
        let name_sequence_effect = v8::String::new(scope, "SequenceEffect").unwrap();
        global.define_own_property(scope, name_sequence_effect.into(), ctor_sequence_effect.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_form_controls_collection) = tmpl_html_form_controls_collection.get_function(scope) {
        let name_html_form_controls_collection = v8::String::new(scope, "HTMLFormControlsCollection").unwrap();
        global.define_own_property(scope, name_html_form_controls_collection.into(), ctor_html_form_controls_collection.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_options_collection) = tmpl_html_options_collection.get_function(scope) {
        let name_html_options_collection = v8::String::new(scope, "HTMLOptionsCollection").unwrap();
        global.define_own_property(scope, name_html_options_collection.into(), ctor_html_options_collection.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_idb_cursor_with_value) = tmpl_idb_cursor_with_value.get_function(scope) {
        let name_idb_cursor_with_value = v8::String::new(scope, "IDBCursorWithValue").unwrap();
        global.define_own_property(scope, name_idb_cursor_with_value.into(), ctor_idb_cursor_with_value.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_css_style_sheet) = tmpl_css_style_sheet.get_function(scope) {
        let name_css_style_sheet = v8::String::new(scope, "CSSStyleSheet").unwrap();
        global.define_own_property(scope, name_css_style_sheet.into(), ctor_css_style_sheet.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_gl_buffer) = tmpl_web_gl_buffer.get_function(scope) {
        let name_web_gl_buffer = v8::String::new(scope, "WebGLBuffer").unwrap();
        global.define_own_property(scope, name_web_gl_buffer.into(), ctor_web_gl_buffer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_gl_framebuffer) = tmpl_web_gl_framebuffer.get_function(scope) {
        let name_web_gl_framebuffer = v8::String::new(scope, "WebGLFramebuffer").unwrap();
        global.define_own_property(scope, name_web_gl_framebuffer.into(), ctor_web_gl_framebuffer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_gl_program) = tmpl_web_gl_program.get_function(scope) {
        let name_web_gl_program = v8::String::new(scope, "WebGLProgram").unwrap();
        global.define_own_property(scope, name_web_gl_program.into(), ctor_web_gl_program.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_gl_query) = tmpl_web_gl_query.get_function(scope) {
        let name_web_gl_query = v8::String::new(scope, "WebGLQuery").unwrap();
        global.define_own_property(scope, name_web_gl_query.into(), ctor_web_gl_query.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_gl_renderbuffer) = tmpl_web_gl_renderbuffer.get_function(scope) {
        let name_web_gl_renderbuffer = v8::String::new(scope, "WebGLRenderbuffer").unwrap();
        global.define_own_property(scope, name_web_gl_renderbuffer.into(), ctor_web_gl_renderbuffer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_gl_sampler) = tmpl_web_gl_sampler.get_function(scope) {
        let name_web_gl_sampler = v8::String::new(scope, "WebGLSampler").unwrap();
        global.define_own_property(scope, name_web_gl_sampler.into(), ctor_web_gl_sampler.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_gl_shader) = tmpl_web_gl_shader.get_function(scope) {
        let name_web_gl_shader = v8::String::new(scope, "WebGLShader").unwrap();
        global.define_own_property(scope, name_web_gl_shader.into(), ctor_web_gl_shader.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_gl_sync) = tmpl_web_gl_sync.get_function(scope) {
        let name_web_gl_sync = v8::String::new(scope, "WebGLSync").unwrap();
        global.define_own_property(scope, name_web_gl_sync.into(), ctor_web_gl_sync.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_gl_texture) = tmpl_web_gl_texture.get_function(scope) {
        let name_web_gl_texture = v8::String::new(scope, "WebGLTexture").unwrap();
        global.define_own_property(scope, name_web_gl_texture.into(), ctor_web_gl_texture.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    // WebGLTimerQueryEXT: NoInterfaceObject — skip global registration
    if let Some(ctor_web_gl_transform_feedback) = tmpl_web_gl_transform_feedback.get_function(scope) {
        let name_web_gl_transform_feedback = v8::String::new(scope, "WebGLTransformFeedback").unwrap();
        global.define_own_property(scope, name_web_gl_transform_feedback.into(), ctor_web_gl_transform_feedback.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_web_gl_vertex_array_object) = tmpl_web_gl_vertex_array_object.get_function(scope) {
        let name_web_gl_vertex_array_object = v8::String::new(scope, "WebGLVertexArrayObject").unwrap();
        global.define_own_property(scope, name_web_gl_vertex_array_object.into(), ctor_web_gl_vertex_array_object.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_xrcpu_depth_information) = tmpl_xrcpu_depth_information.get_function(scope) {
        let name_xrcpu_depth_information = v8::String::new(scope, "XRCPUDepthInformation").unwrap();
        global.define_own_property(scope, name_xrcpu_depth_information.into(), ctor_xrcpu_depth_information.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_web_gl_depth_information) = tmpl_xr_web_gl_depth_information.get_function(scope) {
        let name_xr_web_gl_depth_information = v8::String::new(scope, "XRWebGLDepthInformation").unwrap();
        global.define_own_property(scope, name_xr_web_gl_depth_information.into(), ctor_xr_web_gl_depth_information.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_joint_pose) = tmpl_xr_joint_pose.get_function(scope) {
        let name_xr_joint_pose = v8::String::new(scope, "XRJointPose").unwrap();
        global.define_own_property(scope, name_xr_joint_pose.into(), ctor_xr_joint_pose.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_viewer_pose) = tmpl_xr_viewer_pose.get_function(scope) {
        let name_xr_viewer_pose = v8::String::new(scope, "XRViewerPose").unwrap();
        global.define_own_property(scope, name_xr_viewer_pose.into(), ctor_xr_viewer_pose.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_web_gl_sub_image) = tmpl_xr_web_gl_sub_image.get_function(scope) {
        let name_xr_web_gl_sub_image = v8::String::new(scope, "XRWebGLSubImage").unwrap();
        global.define_own_property(scope, name_xr_web_gl_sub_image.into(), ctor_xr_web_gl_sub_image.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_view_timeline) = tmpl_view_timeline.get_function(scope) {
        let name_view_timeline = v8::String::new(scope, "ViewTimeline").unwrap();
        global.define_own_property(scope, name_view_timeline.into(), ctor_view_timeline.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_apply_block_rule) = tmpl_css_apply_block_rule.get_function(scope) {
        let name_css_apply_block_rule = v8::String::new(scope, "CSSApplyBlockRule").unwrap();
        global.define_own_property(scope, name_css_apply_block_rule.into(), ctor_css_apply_block_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_condition_rule) = tmpl_css_condition_rule.get_function(scope) {
        let name_css_condition_rule = v8::String::new(scope, "CSSConditionRule").unwrap();
        global.define_own_property(scope, name_css_condition_rule.into(), ctor_css_condition_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_contents_block_rule) = tmpl_css_contents_block_rule.get_function(scope) {
        let name_css_contents_block_rule = v8::String::new(scope, "CSSContentsBlockRule").unwrap();
        global.define_own_property(scope, name_css_contents_block_rule.into(), ctor_css_contents_block_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_function_rule) = tmpl_css_function_rule.get_function(scope) {
        let name_css_function_rule = v8::String::new(scope, "CSSFunctionRule").unwrap();
        global.define_own_property(scope, name_css_function_rule.into(), ctor_css_function_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_layer_block_rule) = tmpl_css_layer_block_rule.get_function(scope) {
        let name_css_layer_block_rule = v8::String::new(scope, "CSSLayerBlockRule").unwrap();
        global.define_own_property(scope, name_css_layer_block_rule.into(), ctor_css_layer_block_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_mixin_rule) = tmpl_css_mixin_rule.get_function(scope) {
        let name_css_mixin_rule = v8::String::new(scope, "CSSMixinRule").unwrap();
        global.define_own_property(scope, name_css_mixin_rule.into(), ctor_css_mixin_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_page_rule) = tmpl_css_page_rule.get_function(scope) {
        let name_css_page_rule = v8::String::new(scope, "CSSPageRule").unwrap();
        global.define_own_property(scope, name_css_page_rule.into(), ctor_css_page_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_scope_rule) = tmpl_css_scope_rule.get_function(scope) {
        let name_css_scope_rule = v8::String::new(scope, "CSSScopeRule").unwrap();
        global.define_own_property(scope, name_css_scope_rule.into(), ctor_css_scope_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_starting_style_rule) = tmpl_css_starting_style_rule.get_function(scope) {
        let name_css_starting_style_rule = v8::String::new(scope, "CSSStartingStyleRule").unwrap();
        global.define_own_property(scope, name_css_starting_style_rule.into(), ctor_css_starting_style_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_style_rule) = tmpl_css_style_rule.get_function(scope) {
        let name_css_style_rule = v8::String::new(scope, "CSSStyleRule").unwrap();
        global.define_own_property(scope, name_css_style_rule.into(), ctor_css_style_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_supports_condition_rule) = tmpl_css_supports_condition_rule.get_function(scope) {
        let name_css_supports_condition_rule = v8::String::new(scope, "CSSSupportsConditionRule").unwrap();
        global.define_own_property(scope, name_css_supports_condition_rule.into(), ctor_css_supports_condition_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_color) = tmpl_css_color.get_function(scope) {
        let name_css_color = v8::String::new(scope, "CSSColor").unwrap();
        global.define_own_property(scope, name_css_color.into(), ctor_css_color.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_css_lab) = tmpl_css_lab.get_function(scope) {
        let name_css_lab = v8::String::new(scope, "CSSLab").unwrap();
        global.define_own_property(scope, name_css_lab.into(), ctor_css_lab.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssoklch) = tmpl_cssoklch.get_function(scope) {
        let name_cssoklch = v8::String::new(scope, "CSSOKLCH").unwrap();
        global.define_own_property(scope, name_cssoklch.into(), ctor_cssoklch.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssok_lab) = tmpl_cssok_lab.get_function(scope) {
        let name_cssok_lab = v8::String::new(scope, "CSSOKLab").unwrap();
        global.define_own_property(scope, name_cssok_lab.into(), ctor_cssok_lab.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cssrgb) = tmpl_cssrgb.get_function(scope) {
        let name_cssrgb = v8::String::new(scope, "CSSRGB").unwrap();
        global.define_own_property(scope, name_cssrgb.into(), ctor_cssrgb.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_math_value) = tmpl_css_math_value.get_function(scope) {
        let name_css_math_value = v8::String::new(scope, "CSSMathValue").unwrap();
        global.define_own_property(scope, name_css_math_value.into(), ctor_css_math_value.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_unit_value) = tmpl_css_unit_value.get_function(scope) {
        let name_css_unit_value = v8::String::new(scope, "CSSUnitValue").unwrap();
        global.define_own_property(scope, name_css_unit_value.into(), ctor_css_unit_value.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_css_animation) = tmpl_css_animation.get_function(scope) {
        let name_css_animation = v8::String::new(scope, "CSSAnimation").unwrap();
        global.define_own_property(scope, name_css_animation.into(), ctor_css_animation.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_transition) = tmpl_css_transition.get_function(scope) {
        let name_css_transition = v8::String::new(scope, "CSSTransition").unwrap();
        global.define_own_property(scope, name_css_transition.into(), ctor_css_transition.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_iir_filter_node) = tmpl_iir_filter_node.get_function(scope) {
        let name_iir_filter_node = v8::String::new(scope, "IIRFilterNode").unwrap();
        global.define_own_property(scope, name_iir_filter_node.into(), ctor_iir_filter_node.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_idb_open_db_request) = tmpl_idb_open_db_request.get_function(scope) {
        let name_idb_open_db_request = v8::String::new(scope, "IDBOpenDBRequest").unwrap();
        global.define_own_property(scope, name_idb_open_db_request.into(), ctor_idb_open_db_request.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_midi_input) = tmpl_midi_input.get_function(scope) {
        let name_midi_input = v8::String::new(scope, "MIDIInput").unwrap();
        global.define_own_property(scope, name_midi_input.into(), ctor_midi_input.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_midi_output) = tmpl_midi_output.get_function(scope) {
        let name_midi_output = v8::String::new(scope, "MIDIOutput").unwrap();
        global.define_own_property(scope, name_midi_output.into(), ctor_midi_output.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_bluetooth_le_scan_permission_result) = tmpl_bluetooth_le_scan_permission_result.get_function(scope) {
        let name_bluetooth_le_scan_permission_result = v8::String::new(scope, "BluetoothLEScanPermissionResult").unwrap();
        global.define_own_property(scope, name_bluetooth_le_scan_permission_result.into(), ctor_bluetooth_le_scan_permission_result.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_bluetooth_permission_result) = tmpl_bluetooth_permission_result.get_function(scope) {
        let name_bluetooth_permission_result = v8::String::new(scope, "BluetoothPermissionResult").unwrap();
        global.define_own_property(scope, name_bluetooth_permission_result.into(), ctor_bluetooth_permission_result.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_usb_permission_result) = tmpl_usb_permission_result.get_function(scope) {
        let name_usb_permission_result = v8::String::new(scope, "USBPermissionResult").unwrap();
        global.define_own_property(scope, name_usb_permission_result.into(), ctor_usb_permission_result.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_permission_status) = tmpl_xr_permission_status.get_function(scope) {
        let name_xr_permission_status = v8::String::new(scope, "XRPermissionStatus").unwrap();
        global.define_own_property(scope, name_xr_permission_status.into(), ctor_xr_permission_status.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_vtt_cue) = tmpl_vtt_cue.get_function(scope) {
        let name_vtt_cue = v8::String::new(scope, "VTTCue").unwrap();
        global.define_own_property(scope, name_vtt_cue.into(), ctor_vtt_cue.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_dedicated_worker_global_scope) = tmpl_dedicated_worker_global_scope.get_function(scope) {
        let name_dedicated_worker_global_scope = v8::String::new(scope, "DedicatedWorkerGlobalScope").unwrap();
        global.define_own_property(scope, name_dedicated_worker_global_scope.into(), ctor_dedicated_worker_global_scope.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_rtc_identity_provider_global_scope) = tmpl_rtc_identity_provider_global_scope.get_function(scope) {
        let name_rtc_identity_provider_global_scope = v8::String::new(scope, "RTCIdentityProviderGlobalScope").unwrap();
        global.define_own_property(scope, name_rtc_identity_provider_global_scope.into(), ctor_rtc_identity_provider_global_scope.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_service_worker_global_scope) = tmpl_service_worker_global_scope.get_function(scope) {
        let name_service_worker_global_scope = v8::String::new(scope, "ServiceWorkerGlobalScope").unwrap();
        global.define_own_property(scope, name_service_worker_global_scope.into(), ctor_service_worker_global_scope.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_shared_worker_global_scope) = tmpl_shared_worker_global_scope.get_function(scope) {
        let name_shared_worker_global_scope = v8::String::new(scope, "SharedWorkerGlobalScope").unwrap();
        global.define_own_property(scope, name_shared_worker_global_scope.into(), ctor_shared_worker_global_scope.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xml_http_request) = tmpl_xml_http_request.get_function(scope) {
        let name_xml_http_request = v8::String::new(scope, "XMLHttpRequest").unwrap();
        global.define_own_property(scope, name_xml_http_request.into(), ctor_xml_http_request.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xml_http_request_upload) = tmpl_xml_http_request_upload.get_function(scope) {
        let name_xml_http_request_upload = v8::String::new(scope, "XMLHttpRequestUpload").unwrap();
        global.define_own_property(scope, name_xml_http_request_upload.into(), ctor_xml_http_request_upload.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_composition_layer) = tmpl_xr_composition_layer.get_function(scope) {
        let name_xr_composition_layer = v8::String::new(scope, "XRCompositionLayer").unwrap();
        global.define_own_property(scope, name_xr_composition_layer.into(), ctor_xr_composition_layer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_web_gl_layer) = tmpl_xr_web_gl_layer.get_function(scope) {
        let name_xr_web_gl_layer = v8::String::new(scope, "XRWebGLLayer").unwrap();
        global.define_own_property(scope, name_xr_web_gl_layer.into(), ctor_xr_web_gl_layer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_body_space) = tmpl_xr_body_space.get_function(scope) {
        let name_xr_body_space = v8::String::new(scope, "XRBodySpace").unwrap();
        global.define_own_property(scope, name_xr_body_space.into(), ctor_xr_body_space.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_joint_space) = tmpl_xr_joint_space.get_function(scope) {
        let name_xr_joint_space = v8::String::new(scope, "XRJointSpace").unwrap();
        global.define_own_property(scope, name_xr_joint_space.into(), ctor_xr_joint_space.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_reference_space) = tmpl_xr_reference_space.get_function(scope) {
        let name_xr_reference_space = v8::String::new(scope, "XRReferenceSpace").unwrap();
        global.define_own_property(scope, name_xr_reference_space.into(), ctor_xr_reference_space.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_performance_navigation_timing) = tmpl_performance_navigation_timing.get_function(scope) {
        let name_performance_navigation_timing = v8::String::new(scope, "PerformanceNavigationTiming").unwrap();
        global.define_own_property(scope, name_performance_navigation_timing.into(), ctor_performance_navigation_timing.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_container_rule) = tmpl_css_container_rule.get_function(scope) {
        let name_css_container_rule = v8::String::new(scope, "CSSContainerRule").unwrap();
        global.define_own_property(scope, name_css_container_rule.into(), ctor_css_container_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_media_rule) = tmpl_css_media_rule.get_function(scope) {
        let name_css_media_rule = v8::String::new(scope, "CSSMediaRule").unwrap();
        global.define_own_property(scope, name_css_media_rule.into(), ctor_css_media_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_supports_rule) = tmpl_css_supports_rule.get_function(scope) {
        let name_css_supports_rule = v8::String::new(scope, "CSSSupportsRule").unwrap();
        global.define_own_property(scope, name_css_supports_rule.into(), ctor_css_supports_rule.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_math_clamp) = tmpl_css_math_clamp.get_function(scope) {
        let name_css_math_clamp = v8::String::new(scope, "CSSMathClamp").unwrap();
        global.define_own_property(scope, name_css_math_clamp.into(), ctor_css_math_clamp.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_math_invert) = tmpl_css_math_invert.get_function(scope) {
        let name_css_math_invert = v8::String::new(scope, "CSSMathInvert").unwrap();
        global.define_own_property(scope, name_css_math_invert.into(), ctor_css_math_invert.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_math_max) = tmpl_css_math_max.get_function(scope) {
        let name_css_math_max = v8::String::new(scope, "CSSMathMax").unwrap();
        global.define_own_property(scope, name_css_math_max.into(), ctor_css_math_max.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_math_min) = tmpl_css_math_min.get_function(scope) {
        let name_css_math_min = v8::String::new(scope, "CSSMathMin").unwrap();
        global.define_own_property(scope, name_css_math_min.into(), ctor_css_math_min.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_math_negate) = tmpl_css_math_negate.get_function(scope) {
        let name_css_math_negate = v8::String::new(scope, "CSSMathNegate").unwrap();
        global.define_own_property(scope, name_css_math_negate.into(), ctor_css_math_negate.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_math_product) = tmpl_css_math_product.get_function(scope) {
        let name_css_math_product = v8::String::new(scope, "CSSMathProduct").unwrap();
        global.define_own_property(scope, name_css_math_product.into(), ctor_css_math_product.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_css_math_sum) = tmpl_css_math_sum.get_function(scope) {
        let name_css_math_sum = v8::String::new(scope, "CSSMathSum").unwrap();
        global.define_own_property(scope, name_css_math_sum.into(), ctor_css_math_sum.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_background_fetch_update_ui_event) = tmpl_background_fetch_update_ui_event.get_function(scope) {
        let name_background_fetch_update_ui_event = v8::String::new(scope, "BackgroundFetchUpdateUIEvent").unwrap();
        global.define_own_property(scope, name_background_fetch_update_ui_event.into(), ctor_background_fetch_update_ui_event.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_xml_document) = tmpl_xml_document.get_function(scope) {
        let name_xml_document = v8::String::new(scope, "XMLDocument").unwrap();
        global.define_own_property(scope, name_xml_document.into(), ctor_xml_document.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_shadow_root) = tmpl_shadow_root.get_function(scope) {
        let name_shadow_root = v8::String::new(scope, "ShadowRoot").unwrap();
        global.define_own_property(scope, name_shadow_root.into(), ctor_shadow_root.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_element) = tmpl_html_element.get_function(scope) {
        let name_html_element = v8::String::new(scope, "HTMLElement").unwrap();
        global.define_own_property(scope, name_html_element.into(), ctor_html_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_math_ml_element) = tmpl_math_ml_element.get_function(scope) {
        let name_math_ml_element = v8::String::new(scope, "MathMLElement").unwrap();
        global.define_own_property(scope, name_math_ml_element.into(), ctor_math_ml_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_element) = tmpl_svg_element.get_function(scope) {
        let name_svg_element = v8::String::new(scope, "SVGElement").unwrap();
        global.define_own_property(scope, name_svg_element.into(), ctor_svg_element.into(), v8::PropertyAttribute::DONT_ENUM);
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
    if let Some(ctor_xr_cube_layer) = tmpl_xr_cube_layer.get_function(scope) {
        let name_xr_cube_layer = v8::String::new(scope, "XRCubeLayer").unwrap();
        global.define_own_property(scope, name_xr_cube_layer.into(), ctor_xr_cube_layer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_cylinder_layer) = tmpl_xr_cylinder_layer.get_function(scope) {
        let name_xr_cylinder_layer = v8::String::new(scope, "XRCylinderLayer").unwrap();
        global.define_own_property(scope, name_xr_cylinder_layer.into(), ctor_xr_cylinder_layer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_equirect_layer) = tmpl_xr_equirect_layer.get_function(scope) {
        let name_xr_equirect_layer = v8::String::new(scope, "XREquirectLayer").unwrap();
        global.define_own_property(scope, name_xr_equirect_layer.into(), ctor_xr_equirect_layer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_projection_layer) = tmpl_xr_projection_layer.get_function(scope) {
        let name_xr_projection_layer = v8::String::new(scope, "XRProjectionLayer").unwrap();
        global.define_own_property(scope, name_xr_projection_layer.into(), ctor_xr_projection_layer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_quad_layer) = tmpl_xr_quad_layer.get_function(scope) {
        let name_xr_quad_layer = v8::String::new(scope, "XRQuadLayer").unwrap();
        global.define_own_property(scope, name_xr_quad_layer.into(), ctor_xr_quad_layer.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_xr_bounded_reference_space) = tmpl_xr_bounded_reference_space.get_function(scope) {
        let name_xr_bounded_reference_space = v8::String::new(scope, "XRBoundedReferenceSpace").unwrap();
        global.define_own_property(scope, name_xr_bounded_reference_space.into(), ctor_xr_bounded_reference_space.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_cdata_section) = tmpl_cdata_section.get_function(scope) {
        let name_cdata_section = v8::String::new(scope, "CDATASection").unwrap();
        global.define_own_property(scope, name_cdata_section.into(), ctor_cdata_section.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_use_element_shadow_root) = tmpl_svg_use_element_shadow_root.get_function(scope) {
        let name_svg_use_element_shadow_root = v8::String::new(scope, "SVGUseElementShadowRoot").unwrap();
        global.define_own_property(scope, name_svg_use_element_shadow_root.into(), ctor_svg_use_element_shadow_root.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_anchor_element) = tmpl_html_anchor_element.get_function(scope) {
        let name_html_anchor_element = v8::String::new(scope, "HTMLAnchorElement").unwrap();
        global.define_own_property(scope, name_html_anchor_element.into(), ctor_html_anchor_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_area_element) = tmpl_html_area_element.get_function(scope) {
        let name_html_area_element = v8::String::new(scope, "HTMLAreaElement").unwrap();
        global.define_own_property(scope, name_html_area_element.into(), ctor_html_area_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlbr_element) = tmpl_htmlbr_element.get_function(scope) {
        let name_htmlbr_element = v8::String::new(scope, "HTMLBRElement").unwrap();
        global.define_own_property(scope, name_htmlbr_element.into(), ctor_htmlbr_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_base_element) = tmpl_html_base_element.get_function(scope) {
        let name_html_base_element = v8::String::new(scope, "HTMLBaseElement").unwrap();
        global.define_own_property(scope, name_html_base_element.into(), ctor_html_base_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_body_element) = tmpl_html_body_element.get_function(scope) {
        let name_html_body_element = v8::String::new(scope, "HTMLBodyElement").unwrap();
        global.define_own_property(scope, name_html_body_element.into(), ctor_html_body_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_button_element) = tmpl_html_button_element.get_function(scope) {
        let name_html_button_element = v8::String::new(scope, "HTMLButtonElement").unwrap();
        global.define_own_property(scope, name_html_button_element.into(), ctor_html_button_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_canvas_element) = tmpl_html_canvas_element.get_function(scope) {
        let name_html_canvas_element = v8::String::new(scope, "HTMLCanvasElement").unwrap();
        global.define_own_property(scope, name_html_canvas_element.into(), ctor_html_canvas_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmld_list_element) = tmpl_htmld_list_element.get_function(scope) {
        let name_htmld_list_element = v8::String::new(scope, "HTMLDListElement").unwrap();
        global.define_own_property(scope, name_htmld_list_element.into(), ctor_htmld_list_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_data_element) = tmpl_html_data_element.get_function(scope) {
        let name_html_data_element = v8::String::new(scope, "HTMLDataElement").unwrap();
        global.define_own_property(scope, name_html_data_element.into(), ctor_html_data_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_data_list_element) = tmpl_html_data_list_element.get_function(scope) {
        let name_html_data_list_element = v8::String::new(scope, "HTMLDataListElement").unwrap();
        global.define_own_property(scope, name_html_data_list_element.into(), ctor_html_data_list_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_details_element) = tmpl_html_details_element.get_function(scope) {
        let name_html_details_element = v8::String::new(scope, "HTMLDetailsElement").unwrap();
        global.define_own_property(scope, name_html_details_element.into(), ctor_html_details_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_dialog_element) = tmpl_html_dialog_element.get_function(scope) {
        let name_html_dialog_element = v8::String::new(scope, "HTMLDialogElement").unwrap();
        global.define_own_property(scope, name_html_dialog_element.into(), ctor_html_dialog_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_directory_element) = tmpl_html_directory_element.get_function(scope) {
        let name_html_directory_element = v8::String::new(scope, "HTMLDirectoryElement").unwrap();
        global.define_own_property(scope, name_html_directory_element.into(), ctor_html_directory_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_div_element) = tmpl_html_div_element.get_function(scope) {
        let name_html_div_element = v8::String::new(scope, "HTMLDivElement").unwrap();
        global.define_own_property(scope, name_html_div_element.into(), ctor_html_div_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_embed_element) = tmpl_html_embed_element.get_function(scope) {
        let name_html_embed_element = v8::String::new(scope, "HTMLEmbedElement").unwrap();
        global.define_own_property(scope, name_html_embed_element.into(), ctor_html_embed_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_fenced_frame_element) = tmpl_html_fenced_frame_element.get_function(scope) {
        let name_html_fenced_frame_element = v8::String::new(scope, "HTMLFencedFrameElement").unwrap();
        global.define_own_property(scope, name_html_fenced_frame_element.into(), ctor_html_fenced_frame_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_field_set_element) = tmpl_html_field_set_element.get_function(scope) {
        let name_html_field_set_element = v8::String::new(scope, "HTMLFieldSetElement").unwrap();
        global.define_own_property(scope, name_html_field_set_element.into(), ctor_html_field_set_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_font_element) = tmpl_html_font_element.get_function(scope) {
        let name_html_font_element = v8::String::new(scope, "HTMLFontElement").unwrap();
        global.define_own_property(scope, name_html_font_element.into(), ctor_html_font_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_form_element) = tmpl_html_form_element.get_function(scope) {
        let name_html_form_element = v8::String::new(scope, "HTMLFormElement").unwrap();
        global.define_own_property(scope, name_html_form_element.into(), ctor_html_form_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_frame_element) = tmpl_html_frame_element.get_function(scope) {
        let name_html_frame_element = v8::String::new(scope, "HTMLFrameElement").unwrap();
        global.define_own_property(scope, name_html_frame_element.into(), ctor_html_frame_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_frame_set_element) = tmpl_html_frame_set_element.get_function(scope) {
        let name_html_frame_set_element = v8::String::new(scope, "HTMLFrameSetElement").unwrap();
        global.define_own_property(scope, name_html_frame_set_element.into(), ctor_html_frame_set_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_geolocation_element) = tmpl_html_geolocation_element.get_function(scope) {
        let name_html_geolocation_element = v8::String::new(scope, "HTMLGeolocationElement").unwrap();
        global.define_own_property(scope, name_html_geolocation_element.into(), ctor_html_geolocation_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlhr_element) = tmpl_htmlhr_element.get_function(scope) {
        let name_htmlhr_element = v8::String::new(scope, "HTMLHRElement").unwrap();
        global.define_own_property(scope, name_htmlhr_element.into(), ctor_htmlhr_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_head_element) = tmpl_html_head_element.get_function(scope) {
        let name_html_head_element = v8::String::new(scope, "HTMLHeadElement").unwrap();
        global.define_own_property(scope, name_html_head_element.into(), ctor_html_head_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_heading_element) = tmpl_html_heading_element.get_function(scope) {
        let name_html_heading_element = v8::String::new(scope, "HTMLHeadingElement").unwrap();
        global.define_own_property(scope, name_html_heading_element.into(), ctor_html_heading_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_html_element) = tmpl_html_html_element.get_function(scope) {
        let name_html_html_element = v8::String::new(scope, "HTMLHtmlElement").unwrap();
        global.define_own_property(scope, name_html_html_element.into(), ctor_html_html_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmli_frame_element) = tmpl_htmli_frame_element.get_function(scope) {
        let name_htmli_frame_element = v8::String::new(scope, "HTMLIFrameElement").unwrap();
        global.define_own_property(scope, name_htmli_frame_element.into(), ctor_htmli_frame_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_image_element) = tmpl_html_image_element.get_function(scope) {
        let name_html_image_element = v8::String::new(scope, "HTMLImageElement").unwrap();
        global.define_own_property(scope, name_html_image_element.into(), ctor_html_image_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_input_element) = tmpl_html_input_element.get_function(scope) {
        let name_html_input_element = v8::String::new(scope, "HTMLInputElement").unwrap();
        global.define_own_property(scope, name_html_input_element.into(), ctor_html_input_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlli_element) = tmpl_htmlli_element.get_function(scope) {
        let name_htmlli_element = v8::String::new(scope, "HTMLLIElement").unwrap();
        global.define_own_property(scope, name_htmlli_element.into(), ctor_htmlli_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_label_element) = tmpl_html_label_element.get_function(scope) {
        let name_html_label_element = v8::String::new(scope, "HTMLLabelElement").unwrap();
        global.define_own_property(scope, name_html_label_element.into(), ctor_html_label_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_legend_element) = tmpl_html_legend_element.get_function(scope) {
        let name_html_legend_element = v8::String::new(scope, "HTMLLegendElement").unwrap();
        global.define_own_property(scope, name_html_legend_element.into(), ctor_html_legend_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_link_element) = tmpl_html_link_element.get_function(scope) {
        let name_html_link_element = v8::String::new(scope, "HTMLLinkElement").unwrap();
        global.define_own_property(scope, name_html_link_element.into(), ctor_html_link_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_map_element) = tmpl_html_map_element.get_function(scope) {
        let name_html_map_element = v8::String::new(scope, "HTMLMapElement").unwrap();
        global.define_own_property(scope, name_html_map_element.into(), ctor_html_map_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_marquee_element) = tmpl_html_marquee_element.get_function(scope) {
        let name_html_marquee_element = v8::String::new(scope, "HTMLMarqueeElement").unwrap();
        global.define_own_property(scope, name_html_marquee_element.into(), ctor_html_marquee_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_media_element) = tmpl_html_media_element.get_function(scope) {
        let name_html_media_element = v8::String::new(scope, "HTMLMediaElement").unwrap();
        global.define_own_property(scope, name_html_media_element.into(), ctor_html_media_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_menu_element) = tmpl_html_menu_element.get_function(scope) {
        let name_html_menu_element = v8::String::new(scope, "HTMLMenuElement").unwrap();
        global.define_own_property(scope, name_html_menu_element.into(), ctor_html_menu_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_meta_element) = tmpl_html_meta_element.get_function(scope) {
        let name_html_meta_element = v8::String::new(scope, "HTMLMetaElement").unwrap();
        global.define_own_property(scope, name_html_meta_element.into(), ctor_html_meta_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_meter_element) = tmpl_html_meter_element.get_function(scope) {
        let name_html_meter_element = v8::String::new(scope, "HTMLMeterElement").unwrap();
        global.define_own_property(scope, name_html_meter_element.into(), ctor_html_meter_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_mod_element) = tmpl_html_mod_element.get_function(scope) {
        let name_html_mod_element = v8::String::new(scope, "HTMLModElement").unwrap();
        global.define_own_property(scope, name_html_mod_element.into(), ctor_html_mod_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_model_element) = tmpl_html_model_element.get_function(scope) {
        let name_html_model_element = v8::String::new(scope, "HTMLModelElement").unwrap();
        global.define_own_property(scope, name_html_model_element.into(), ctor_html_model_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlo_list_element) = tmpl_htmlo_list_element.get_function(scope) {
        let name_htmlo_list_element = v8::String::new(scope, "HTMLOListElement").unwrap();
        global.define_own_property(scope, name_htmlo_list_element.into(), ctor_htmlo_list_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_object_element) = tmpl_html_object_element.get_function(scope) {
        let name_html_object_element = v8::String::new(scope, "HTMLObjectElement").unwrap();
        global.define_own_property(scope, name_html_object_element.into(), ctor_html_object_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_opt_group_element) = tmpl_html_opt_group_element.get_function(scope) {
        let name_html_opt_group_element = v8::String::new(scope, "HTMLOptGroupElement").unwrap();
        global.define_own_property(scope, name_html_opt_group_element.into(), ctor_html_opt_group_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_option_element) = tmpl_html_option_element.get_function(scope) {
        let name_html_option_element = v8::String::new(scope, "HTMLOptionElement").unwrap();
        global.define_own_property(scope, name_html_option_element.into(), ctor_html_option_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_output_element) = tmpl_html_output_element.get_function(scope) {
        let name_html_output_element = v8::String::new(scope, "HTMLOutputElement").unwrap();
        global.define_own_property(scope, name_html_output_element.into(), ctor_html_output_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_paragraph_element) = tmpl_html_paragraph_element.get_function(scope) {
        let name_html_paragraph_element = v8::String::new(scope, "HTMLParagraphElement").unwrap();
        global.define_own_property(scope, name_html_paragraph_element.into(), ctor_html_paragraph_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_param_element) = tmpl_html_param_element.get_function(scope) {
        let name_html_param_element = v8::String::new(scope, "HTMLParamElement").unwrap();
        global.define_own_property(scope, name_html_param_element.into(), ctor_html_param_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_picture_element) = tmpl_html_picture_element.get_function(scope) {
        let name_html_picture_element = v8::String::new(scope, "HTMLPictureElement").unwrap();
        global.define_own_property(scope, name_html_picture_element.into(), ctor_html_picture_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_portal_element) = tmpl_html_portal_element.get_function(scope) {
        let name_html_portal_element = v8::String::new(scope, "HTMLPortalElement").unwrap();
        global.define_own_property(scope, name_html_portal_element.into(), ctor_html_portal_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_pre_element) = tmpl_html_pre_element.get_function(scope) {
        let name_html_pre_element = v8::String::new(scope, "HTMLPreElement").unwrap();
        global.define_own_property(scope, name_html_pre_element.into(), ctor_html_pre_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_progress_element) = tmpl_html_progress_element.get_function(scope) {
        let name_html_progress_element = v8::String::new(scope, "HTMLProgressElement").unwrap();
        global.define_own_property(scope, name_html_progress_element.into(), ctor_html_progress_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_quote_element) = tmpl_html_quote_element.get_function(scope) {
        let name_html_quote_element = v8::String::new(scope, "HTMLQuoteElement").unwrap();
        global.define_own_property(scope, name_html_quote_element.into(), ctor_html_quote_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_script_element) = tmpl_html_script_element.get_function(scope) {
        let name_html_script_element = v8::String::new(scope, "HTMLScriptElement").unwrap();
        global.define_own_property(scope, name_html_script_element.into(), ctor_html_script_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_select_element) = tmpl_html_select_element.get_function(scope) {
        let name_html_select_element = v8::String::new(scope, "HTMLSelectElement").unwrap();
        global.define_own_property(scope, name_html_select_element.into(), ctor_html_select_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_selected_content_element) = tmpl_html_selected_content_element.get_function(scope) {
        let name_html_selected_content_element = v8::String::new(scope, "HTMLSelectedContentElement").unwrap();
        global.define_own_property(scope, name_html_selected_content_element.into(), ctor_html_selected_content_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_slot_element) = tmpl_html_slot_element.get_function(scope) {
        let name_html_slot_element = v8::String::new(scope, "HTMLSlotElement").unwrap();
        global.define_own_property(scope, name_html_slot_element.into(), ctor_html_slot_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_source_element) = tmpl_html_source_element.get_function(scope) {
        let name_html_source_element = v8::String::new(scope, "HTMLSourceElement").unwrap();
        global.define_own_property(scope, name_html_source_element.into(), ctor_html_source_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_span_element) = tmpl_html_span_element.get_function(scope) {
        let name_html_span_element = v8::String::new(scope, "HTMLSpanElement").unwrap();
        global.define_own_property(scope, name_html_span_element.into(), ctor_html_span_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_style_element) = tmpl_html_style_element.get_function(scope) {
        let name_html_style_element = v8::String::new(scope, "HTMLStyleElement").unwrap();
        global.define_own_property(scope, name_html_style_element.into(), ctor_html_style_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_table_caption_element) = tmpl_html_table_caption_element.get_function(scope) {
        let name_html_table_caption_element = v8::String::new(scope, "HTMLTableCaptionElement").unwrap();
        global.define_own_property(scope, name_html_table_caption_element.into(), ctor_html_table_caption_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_table_cell_element) = tmpl_html_table_cell_element.get_function(scope) {
        let name_html_table_cell_element = v8::String::new(scope, "HTMLTableCellElement").unwrap();
        global.define_own_property(scope, name_html_table_cell_element.into(), ctor_html_table_cell_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_table_col_element) = tmpl_html_table_col_element.get_function(scope) {
        let name_html_table_col_element = v8::String::new(scope, "HTMLTableColElement").unwrap();
        global.define_own_property(scope, name_html_table_col_element.into(), ctor_html_table_col_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_table_element) = tmpl_html_table_element.get_function(scope) {
        let name_html_table_element = v8::String::new(scope, "HTMLTableElement").unwrap();
        global.define_own_property(scope, name_html_table_element.into(), ctor_html_table_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_table_row_element) = tmpl_html_table_row_element.get_function(scope) {
        let name_html_table_row_element = v8::String::new(scope, "HTMLTableRowElement").unwrap();
        global.define_own_property(scope, name_html_table_row_element.into(), ctor_html_table_row_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_table_section_element) = tmpl_html_table_section_element.get_function(scope) {
        let name_html_table_section_element = v8::String::new(scope, "HTMLTableSectionElement").unwrap();
        global.define_own_property(scope, name_html_table_section_element.into(), ctor_html_table_section_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_template_element) = tmpl_html_template_element.get_function(scope) {
        let name_html_template_element = v8::String::new(scope, "HTMLTemplateElement").unwrap();
        global.define_own_property(scope, name_html_template_element.into(), ctor_html_template_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_text_area_element) = tmpl_html_text_area_element.get_function(scope) {
        let name_html_text_area_element = v8::String::new(scope, "HTMLTextAreaElement").unwrap();
        global.define_own_property(scope, name_html_text_area_element.into(), ctor_html_text_area_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_time_element) = tmpl_html_time_element.get_function(scope) {
        let name_html_time_element = v8::String::new(scope, "HTMLTimeElement").unwrap();
        global.define_own_property(scope, name_html_time_element.into(), ctor_html_time_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_title_element) = tmpl_html_title_element.get_function(scope) {
        let name_html_title_element = v8::String::new(scope, "HTMLTitleElement").unwrap();
        global.define_own_property(scope, name_html_title_element.into(), ctor_html_title_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_track_element) = tmpl_html_track_element.get_function(scope) {
        let name_html_track_element = v8::String::new(scope, "HTMLTrackElement").unwrap();
        global.define_own_property(scope, name_html_track_element.into(), ctor_html_track_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_htmlu_list_element) = tmpl_htmlu_list_element.get_function(scope) {
        let name_htmlu_list_element = v8::String::new(scope, "HTMLUListElement").unwrap();
        global.define_own_property(scope, name_htmlu_list_element.into(), ctor_htmlu_list_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_unknown_element) = tmpl_html_unknown_element.get_function(scope) {
        let name_html_unknown_element = v8::String::new(scope, "HTMLUnknownElement").unwrap();
        global.define_own_property(scope, name_html_unknown_element.into(), ctor_html_unknown_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_math_ml_anchor_element) = tmpl_math_ml_anchor_element.get_function(scope) {
        let name_math_ml_anchor_element = v8::String::new(scope, "MathMLAnchorElement").unwrap();
        global.define_own_property(scope, name_math_ml_anchor_element.into(), ctor_math_ml_anchor_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_animation_element) = tmpl_svg_animation_element.get_function(scope) {
        let name_svg_animation_element = v8::String::new(scope, "SVGAnimationElement").unwrap();
        global.define_own_property(scope, name_svg_animation_element.into(), ctor_svg_animation_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_clip_path_element) = tmpl_svg_clip_path_element.get_function(scope) {
        let name_svg_clip_path_element = v8::String::new(scope, "SVGClipPathElement").unwrap();
        global.define_own_property(scope, name_svg_clip_path_element.into(), ctor_svg_clip_path_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_component_transfer_function_element) = tmpl_svg_component_transfer_function_element.get_function(scope) {
        let name_svg_component_transfer_function_element = v8::String::new(scope, "SVGComponentTransferFunctionElement").unwrap();
        global.define_own_property(scope, name_svg_component_transfer_function_element.into(), ctor_svg_component_transfer_function_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_desc_element) = tmpl_svg_desc_element.get_function(scope) {
        let name_svg_desc_element = v8::String::new(scope, "SVGDescElement").unwrap();
        global.define_own_property(scope, name_svg_desc_element.into(), ctor_svg_desc_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_blend_element) = tmpl_svgfe_blend_element.get_function(scope) {
        let name_svgfe_blend_element = v8::String::new(scope, "SVGFEBlendElement").unwrap();
        global.define_own_property(scope, name_svgfe_blend_element.into(), ctor_svgfe_blend_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_color_matrix_element) = tmpl_svgfe_color_matrix_element.get_function(scope) {
        let name_svgfe_color_matrix_element = v8::String::new(scope, "SVGFEColorMatrixElement").unwrap();
        global.define_own_property(scope, name_svgfe_color_matrix_element.into(), ctor_svgfe_color_matrix_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_component_transfer_element) = tmpl_svgfe_component_transfer_element.get_function(scope) {
        let name_svgfe_component_transfer_element = v8::String::new(scope, "SVGFEComponentTransferElement").unwrap();
        global.define_own_property(scope, name_svgfe_component_transfer_element.into(), ctor_svgfe_component_transfer_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_composite_element) = tmpl_svgfe_composite_element.get_function(scope) {
        let name_svgfe_composite_element = v8::String::new(scope, "SVGFECompositeElement").unwrap();
        global.define_own_property(scope, name_svgfe_composite_element.into(), ctor_svgfe_composite_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_convolve_matrix_element) = tmpl_svgfe_convolve_matrix_element.get_function(scope) {
        let name_svgfe_convolve_matrix_element = v8::String::new(scope, "SVGFEConvolveMatrixElement").unwrap();
        global.define_own_property(scope, name_svgfe_convolve_matrix_element.into(), ctor_svgfe_convolve_matrix_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_diffuse_lighting_element) = tmpl_svgfe_diffuse_lighting_element.get_function(scope) {
        let name_svgfe_diffuse_lighting_element = v8::String::new(scope, "SVGFEDiffuseLightingElement").unwrap();
        global.define_own_property(scope, name_svgfe_diffuse_lighting_element.into(), ctor_svgfe_diffuse_lighting_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_displacement_map_element) = tmpl_svgfe_displacement_map_element.get_function(scope) {
        let name_svgfe_displacement_map_element = v8::String::new(scope, "SVGFEDisplacementMapElement").unwrap();
        global.define_own_property(scope, name_svgfe_displacement_map_element.into(), ctor_svgfe_displacement_map_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_distant_light_element) = tmpl_svgfe_distant_light_element.get_function(scope) {
        let name_svgfe_distant_light_element = v8::String::new(scope, "SVGFEDistantLightElement").unwrap();
        global.define_own_property(scope, name_svgfe_distant_light_element.into(), ctor_svgfe_distant_light_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_drop_shadow_element) = tmpl_svgfe_drop_shadow_element.get_function(scope) {
        let name_svgfe_drop_shadow_element = v8::String::new(scope, "SVGFEDropShadowElement").unwrap();
        global.define_own_property(scope, name_svgfe_drop_shadow_element.into(), ctor_svgfe_drop_shadow_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_flood_element) = tmpl_svgfe_flood_element.get_function(scope) {
        let name_svgfe_flood_element = v8::String::new(scope, "SVGFEFloodElement").unwrap();
        global.define_own_property(scope, name_svgfe_flood_element.into(), ctor_svgfe_flood_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_gaussian_blur_element) = tmpl_svgfe_gaussian_blur_element.get_function(scope) {
        let name_svgfe_gaussian_blur_element = v8::String::new(scope, "SVGFEGaussianBlurElement").unwrap();
        global.define_own_property(scope, name_svgfe_gaussian_blur_element.into(), ctor_svgfe_gaussian_blur_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_image_element) = tmpl_svgfe_image_element.get_function(scope) {
        let name_svgfe_image_element = v8::String::new(scope, "SVGFEImageElement").unwrap();
        global.define_own_property(scope, name_svgfe_image_element.into(), ctor_svgfe_image_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_merge_element) = tmpl_svgfe_merge_element.get_function(scope) {
        let name_svgfe_merge_element = v8::String::new(scope, "SVGFEMergeElement").unwrap();
        global.define_own_property(scope, name_svgfe_merge_element.into(), ctor_svgfe_merge_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_merge_node_element) = tmpl_svgfe_merge_node_element.get_function(scope) {
        let name_svgfe_merge_node_element = v8::String::new(scope, "SVGFEMergeNodeElement").unwrap();
        global.define_own_property(scope, name_svgfe_merge_node_element.into(), ctor_svgfe_merge_node_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_morphology_element) = tmpl_svgfe_morphology_element.get_function(scope) {
        let name_svgfe_morphology_element = v8::String::new(scope, "SVGFEMorphologyElement").unwrap();
        global.define_own_property(scope, name_svgfe_morphology_element.into(), ctor_svgfe_morphology_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_offset_element) = tmpl_svgfe_offset_element.get_function(scope) {
        let name_svgfe_offset_element = v8::String::new(scope, "SVGFEOffsetElement").unwrap();
        global.define_own_property(scope, name_svgfe_offset_element.into(), ctor_svgfe_offset_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_point_light_element) = tmpl_svgfe_point_light_element.get_function(scope) {
        let name_svgfe_point_light_element = v8::String::new(scope, "SVGFEPointLightElement").unwrap();
        global.define_own_property(scope, name_svgfe_point_light_element.into(), ctor_svgfe_point_light_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_specular_lighting_element) = tmpl_svgfe_specular_lighting_element.get_function(scope) {
        let name_svgfe_specular_lighting_element = v8::String::new(scope, "SVGFESpecularLightingElement").unwrap();
        global.define_own_property(scope, name_svgfe_specular_lighting_element.into(), ctor_svgfe_specular_lighting_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_spot_light_element) = tmpl_svgfe_spot_light_element.get_function(scope) {
        let name_svgfe_spot_light_element = v8::String::new(scope, "SVGFESpotLightElement").unwrap();
        global.define_own_property(scope, name_svgfe_spot_light_element.into(), ctor_svgfe_spot_light_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_tile_element) = tmpl_svgfe_tile_element.get_function(scope) {
        let name_svgfe_tile_element = v8::String::new(scope, "SVGFETileElement").unwrap();
        global.define_own_property(scope, name_svgfe_tile_element.into(), ctor_svgfe_tile_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_turbulence_element) = tmpl_svgfe_turbulence_element.get_function(scope) {
        let name_svgfe_turbulence_element = v8::String::new(scope, "SVGFETurbulenceElement").unwrap();
        global.define_own_property(scope, name_svgfe_turbulence_element.into(), ctor_svgfe_turbulence_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_filter_element) = tmpl_svg_filter_element.get_function(scope) {
        let name_svg_filter_element = v8::String::new(scope, "SVGFilterElement").unwrap();
        global.define_own_property(scope, name_svg_filter_element.into(), ctor_svg_filter_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_gradient_element) = tmpl_svg_gradient_element.get_function(scope) {
        let name_svg_gradient_element = v8::String::new(scope, "SVGGradientElement").unwrap();
        global.define_own_property(scope, name_svg_gradient_element.into(), ctor_svg_gradient_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_graphics_element) = tmpl_svg_graphics_element.get_function(scope) {
        let name_svg_graphics_element = v8::String::new(scope, "SVGGraphicsElement").unwrap();
        global.define_own_property(scope, name_svg_graphics_element.into(), ctor_svg_graphics_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgm_path_element) = tmpl_svgm_path_element.get_function(scope) {
        let name_svgm_path_element = v8::String::new(scope, "SVGMPathElement").unwrap();
        global.define_own_property(scope, name_svgm_path_element.into(), ctor_svgm_path_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_marker_element) = tmpl_svg_marker_element.get_function(scope) {
        let name_svg_marker_element = v8::String::new(scope, "SVGMarkerElement").unwrap();
        global.define_own_property(scope, name_svg_marker_element.into(), ctor_svg_marker_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_mask_element) = tmpl_svg_mask_element.get_function(scope) {
        let name_svg_mask_element = v8::String::new(scope, "SVGMaskElement").unwrap();
        global.define_own_property(scope, name_svg_mask_element.into(), ctor_svg_mask_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_metadata_element) = tmpl_svg_metadata_element.get_function(scope) {
        let name_svg_metadata_element = v8::String::new(scope, "SVGMetadataElement").unwrap();
        global.define_own_property(scope, name_svg_metadata_element.into(), ctor_svg_metadata_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_pattern_element) = tmpl_svg_pattern_element.get_function(scope) {
        let name_svg_pattern_element = v8::String::new(scope, "SVGPatternElement").unwrap();
        global.define_own_property(scope, name_svg_pattern_element.into(), ctor_svg_pattern_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_script_element) = tmpl_svg_script_element.get_function(scope) {
        let name_svg_script_element = v8::String::new(scope, "SVGScriptElement").unwrap();
        global.define_own_property(scope, name_svg_script_element.into(), ctor_svg_script_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_stop_element) = tmpl_svg_stop_element.get_function(scope) {
        let name_svg_stop_element = v8::String::new(scope, "SVGStopElement").unwrap();
        global.define_own_property(scope, name_svg_stop_element.into(), ctor_svg_stop_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_style_element) = tmpl_svg_style_element.get_function(scope) {
        let name_svg_style_element = v8::String::new(scope, "SVGStyleElement").unwrap();
        global.define_own_property(scope, name_svg_style_element.into(), ctor_svg_style_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_title_element) = tmpl_svg_title_element.get_function(scope) {
        let name_svg_title_element = v8::String::new(scope, "SVGTitleElement").unwrap();
        global.define_own_property(scope, name_svg_title_element.into(), ctor_svg_title_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_view_element) = tmpl_svg_view_element.get_function(scope) {
        let name_svg_view_element = v8::String::new(scope, "SVGViewElement").unwrap();
        global.define_own_property(scope, name_svg_view_element.into(), ctor_svg_view_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_audio_element) = tmpl_html_audio_element.get_function(scope) {
        let name_html_audio_element = v8::String::new(scope, "HTMLAudioElement").unwrap();
        global.define_own_property(scope, name_html_audio_element.into(), ctor_html_audio_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_html_video_element) = tmpl_html_video_element.get_function(scope) {
        let name_html_video_element = v8::String::new(scope, "HTMLVideoElement").unwrap();
        global.define_own_property(scope, name_html_video_element.into(), ctor_html_video_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_animate_element) = tmpl_svg_animate_element.get_function(scope) {
        let name_svg_animate_element = v8::String::new(scope, "SVGAnimateElement").unwrap();
        global.define_own_property(scope, name_svg_animate_element.into(), ctor_svg_animate_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_animate_motion_element) = tmpl_svg_animate_motion_element.get_function(scope) {
        let name_svg_animate_motion_element = v8::String::new(scope, "SVGAnimateMotionElement").unwrap();
        global.define_own_property(scope, name_svg_animate_motion_element.into(), ctor_svg_animate_motion_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_animate_transform_element) = tmpl_svg_animate_transform_element.get_function(scope) {
        let name_svg_animate_transform_element = v8::String::new(scope, "SVGAnimateTransformElement").unwrap();
        global.define_own_property(scope, name_svg_animate_transform_element.into(), ctor_svg_animate_transform_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_set_element) = tmpl_svg_set_element.get_function(scope) {
        let name_svg_set_element = v8::String::new(scope, "SVGSetElement").unwrap();
        global.define_own_property(scope, name_svg_set_element.into(), ctor_svg_set_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_func_a_element) = tmpl_svgfe_func_a_element.get_function(scope) {
        let name_svgfe_func_a_element = v8::String::new(scope, "SVGFEFuncAElement").unwrap();
        global.define_own_property(scope, name_svgfe_func_a_element.into(), ctor_svgfe_func_a_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_func_b_element) = tmpl_svgfe_func_b_element.get_function(scope) {
        let name_svgfe_func_b_element = v8::String::new(scope, "SVGFEFuncBElement").unwrap();
        global.define_own_property(scope, name_svgfe_func_b_element.into(), ctor_svgfe_func_b_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_func_g_element) = tmpl_svgfe_func_g_element.get_function(scope) {
        let name_svgfe_func_g_element = v8::String::new(scope, "SVGFEFuncGElement").unwrap();
        global.define_own_property(scope, name_svgfe_func_g_element.into(), ctor_svgfe_func_g_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgfe_func_r_element) = tmpl_svgfe_func_r_element.get_function(scope) {
        let name_svgfe_func_r_element = v8::String::new(scope, "SVGFEFuncRElement").unwrap();
        global.define_own_property(scope, name_svgfe_func_r_element.into(), ctor_svgfe_func_r_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_linear_gradient_element) = tmpl_svg_linear_gradient_element.get_function(scope) {
        let name_svg_linear_gradient_element = v8::String::new(scope, "SVGLinearGradientElement").unwrap();
        global.define_own_property(scope, name_svg_linear_gradient_element.into(), ctor_svg_linear_gradient_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_radial_gradient_element) = tmpl_svg_radial_gradient_element.get_function(scope) {
        let name_svg_radial_gradient_element = v8::String::new(scope, "SVGRadialGradientElement").unwrap();
        global.define_own_property(scope, name_svg_radial_gradient_element.into(), ctor_svg_radial_gradient_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svga_element) = tmpl_svga_element.get_function(scope) {
        let name_svga_element = v8::String::new(scope, "SVGAElement").unwrap();
        global.define_own_property(scope, name_svga_element.into(), ctor_svga_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_defs_element) = tmpl_svg_defs_element.get_function(scope) {
        let name_svg_defs_element = v8::String::new(scope, "SVGDefsElement").unwrap();
        global.define_own_property(scope, name_svg_defs_element.into(), ctor_svg_defs_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_foreign_object_element) = tmpl_svg_foreign_object_element.get_function(scope) {
        let name_svg_foreign_object_element = v8::String::new(scope, "SVGForeignObjectElement").unwrap();
        global.define_own_property(scope, name_svg_foreign_object_element.into(), ctor_svg_foreign_object_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgg_element) = tmpl_svgg_element.get_function(scope) {
        let name_svgg_element = v8::String::new(scope, "SVGGElement").unwrap();
        global.define_own_property(scope, name_svgg_element.into(), ctor_svgg_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_geometry_element) = tmpl_svg_geometry_element.get_function(scope) {
        let name_svg_geometry_element = v8::String::new(scope, "SVGGeometryElement").unwrap();
        global.define_own_property(scope, name_svg_geometry_element.into(), ctor_svg_geometry_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_image_element) = tmpl_svg_image_element.get_function(scope) {
        let name_svg_image_element = v8::String::new(scope, "SVGImageElement").unwrap();
        global.define_own_property(scope, name_svg_image_element.into(), ctor_svg_image_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgsvg_element) = tmpl_svgsvg_element.get_function(scope) {
        let name_svgsvg_element = v8::String::new(scope, "SVGSVGElement").unwrap();
        global.define_own_property(scope, name_svgsvg_element.into(), ctor_svgsvg_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_switch_element) = tmpl_svg_switch_element.get_function(scope) {
        let name_svg_switch_element = v8::String::new(scope, "SVGSwitchElement").unwrap();
        global.define_own_property(scope, name_svg_switch_element.into(), ctor_svg_switch_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_symbol_element) = tmpl_svg_symbol_element.get_function(scope) {
        let name_svg_symbol_element = v8::String::new(scope, "SVGSymbolElement").unwrap();
        global.define_own_property(scope, name_svg_symbol_element.into(), ctor_svg_symbol_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_text_content_element) = tmpl_svg_text_content_element.get_function(scope) {
        let name_svg_text_content_element = v8::String::new(scope, "SVGTextContentElement").unwrap();
        global.define_own_property(scope, name_svg_text_content_element.into(), ctor_svg_text_content_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_use_element) = tmpl_svg_use_element.get_function(scope) {
        let name_svg_use_element = v8::String::new(scope, "SVGUseElement").unwrap();
        global.define_own_property(scope, name_svg_use_element.into(), ctor_svg_use_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_circle_element) = tmpl_svg_circle_element.get_function(scope) {
        let name_svg_circle_element = v8::String::new(scope, "SVGCircleElement").unwrap();
        global.define_own_property(scope, name_svg_circle_element.into(), ctor_svg_circle_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_ellipse_element) = tmpl_svg_ellipse_element.get_function(scope) {
        let name_svg_ellipse_element = v8::String::new(scope, "SVGEllipseElement").unwrap();
        global.define_own_property(scope, name_svg_ellipse_element.into(), ctor_svg_ellipse_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_line_element) = tmpl_svg_line_element.get_function(scope) {
        let name_svg_line_element = v8::String::new(scope, "SVGLineElement").unwrap();
        global.define_own_property(scope, name_svg_line_element.into(), ctor_svg_line_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_path_element) = tmpl_svg_path_element.get_function(scope) {
        let name_svg_path_element = v8::String::new(scope, "SVGPathElement").unwrap();
        global.define_own_property(scope, name_svg_path_element.into(), ctor_svg_path_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_polygon_element) = tmpl_svg_polygon_element.get_function(scope) {
        let name_svg_polygon_element = v8::String::new(scope, "SVGPolygonElement").unwrap();
        global.define_own_property(scope, name_svg_polygon_element.into(), ctor_svg_polygon_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_polyline_element) = tmpl_svg_polyline_element.get_function(scope) {
        let name_svg_polyline_element = v8::String::new(scope, "SVGPolylineElement").unwrap();
        global.define_own_property(scope, name_svg_polyline_element.into(), ctor_svg_polyline_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_rect_element) = tmpl_svg_rect_element.get_function(scope) {
        let name_svg_rect_element = v8::String::new(scope, "SVGRectElement").unwrap();
        global.define_own_property(scope, name_svg_rect_element.into(), ctor_svg_rect_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_text_path_element) = tmpl_svg_text_path_element.get_function(scope) {
        let name_svg_text_path_element = v8::String::new(scope, "SVGTextPathElement").unwrap();
        global.define_own_property(scope, name_svg_text_path_element.into(), ctor_svg_text_path_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_text_positioning_element) = tmpl_svg_text_positioning_element.get_function(scope) {
        let name_svg_text_positioning_element = v8::String::new(scope, "SVGTextPositioningElement").unwrap();
        global.define_own_property(scope, name_svg_text_positioning_element.into(), ctor_svg_text_positioning_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svgt_span_element) = tmpl_svgt_span_element.get_function(scope) {
        let name_svgt_span_element = v8::String::new(scope, "SVGTSpanElement").unwrap();
        global.define_own_property(scope, name_svgt_span_element.into(), ctor_svgt_span_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
    if let Some(ctor_svg_text_element) = tmpl_svg_text_element.get_function(scope) {
        let name_svg_text_element = v8::String::new(scope, "SVGTextElement").unwrap();
        global.define_own_property(scope, name_svg_text_element.into(), ctor_svg_text_element.into(), v8::PropertyAttribute::DONT_ENUM);
    }
}
