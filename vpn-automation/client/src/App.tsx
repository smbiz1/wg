import { createSignal, createEffect, Show, For, onMount } from 'solid-js';
import { invoke } from '@tauri-apps/api/tauri';
import ApiKeyInput from './components/ApiKeyInput';
import ConnectionToggle from './components/ConnectionToggle';
import ServerSelector from './components/ServerSelector';
import './App.css';

interface Server {
  id: string;
  name: string;
  endpoint: string;
  port: number;
  region: string;
  country: string;
  tier: string;
  latitude: number;
  longitude: number;
  latency?: number;
}

interface ConnectionInfo {
  status: string;
  server?: Server;
}

export default function App() {
  const [hasApiKey, setHasApiKey] = createSignal(false);
  const [isConnected, setIsConnected] = createSignal(false);
  const [isConnecting, setIsConnecting] = createSignal(false);
  const [currentServer, setCurrentServer] = createSignal<Server | null>(null);
  const [servers, setServers] = createSignal<Server[]>([]);
  const [selectedServerId, setSelectedServerId] = createSignal<string | null>(null);
  const [error, setError] = createSignal<string | null>(null);
  const [wireguardInstalled, setWireguardInstalled] = createSignal(false);

  onMount(async () => {
    // Check if WireGuard is installed
    try {
      const installed = await invoke<boolean>('check_wireguard_installed');
      setWireguardInstalled(installed);
    } catch (err) {
      console.error('Failed to check WireGuard installation:', err);
    }

    // Check connection status on mount
    await updateConnectionStatus();
  });

  const updateConnectionStatus = async () => {
    try {
      const status = await invoke<ConnectionInfo>('get_connection_status');

      if (status.status === 'Connected' && status.server) {
        setIsConnected(true);
        setCurrentServer(status.server);
      } else {
        setIsConnected(false);
        setCurrentServer(null);
      }
    } catch (err) {
      console.error('Failed to get connection status:', err);
    }
  };

  const handleApiKeySubmit = async (apiKey: string) => {
    setError(null);

    try {
      const isValid = await invoke<boolean>('save_api_key', { apiKey });

      if (isValid) {
        setHasApiKey(true);
        await loadServers();
      } else {
        setError('Invalid API key. Please check and try again.');
      }
    } catch (err) {
      setError(`Failed to validate API key: ${err}`);
    }
  };

  const loadServers = async () => {
    setError(null);

    try {
      const serverList = await invoke<Server[]>('fetch_servers');
      setServers(serverList);

      // Auto-select best server (first one, already sorted by latency)
      if (serverList.length > 0 && !selectedServerId()) {
        setSelectedServerId(serverList[0].id);
      }
    } catch (err) {
      setError(`Failed to load servers: ${err}`);
    }
  };

  const handleConnect = async () => {
    setError(null);
    setIsConnecting(true);

    try {
      const connectionInfo = await invoke<ConnectionInfo>('connect_vpn', {
        serverId: selectedServerId(),
      });

      if (connectionInfo.status === 'Connected' && connectionInfo.server) {
        setIsConnected(true);
        setCurrentServer(connectionInfo.server);
      }
    } catch (err) {
      setError(`Failed to connect: ${err}`);
    } finally {
      setIsConnecting(false);
    }
  };

  const handleDisconnect = async () => {
    setError(null);
    setIsConnecting(true);

    try {
      await invoke('disconnect_vpn');
      setIsConnected(false);
      setCurrentServer(null);
    } catch (err) {
      setError(`Failed to disconnect: ${err}`);
    } finally {
      setIsConnecting(false);
    }
  };

  return (
    <div class="app">
      <div class="app-header">
        <h1>Private VPN</h1>
        <p class="subtitle">Secure your connection</p>
      </div>

      <div class="app-content">
        <Show when={!wireguardInstalled()}>
          <div class="warning-banner">
            <strong>⚠️ WireGuard Not Installed</strong>
            <p>Please install WireGuard to use this application.</p>
            <a href="https://www.wireguard.com/install/" target="_blank">
              Installation Instructions
            </a>
          </div>
        </Show>

        <Show when={error()}>
          <div class="error-banner">
            <strong>Error:</strong> {error()}
          </div>
        </Show>

        <Show
          when={hasApiKey()}
          fallback={<ApiKeyInput onSubmit={handleApiKeySubmit} />}
        >
          <div class="vpn-controls">
            <ConnectionToggle
              isConnected={isConnected()}
              isConnecting={isConnecting()}
              currentServer={currentServer()}
              onConnect={handleConnect}
              onDisconnect={handleDisconnect}
            />

            <Show when={!isConnected()}>
              <ServerSelector
                servers={servers()}
                selectedId={selectedServerId()}
                onSelect={setSelectedServerId}
              />
            </Show>

            <Show when={isConnected() && currentServer()}>
              <div class="connection-info">
                <div class="info-item">
                  <span class="label">Server:</span>
                  <span class="value">{currentServer()?.name}</span>
                </div>
                <div class="info-item">
                  <span class="label">Region:</span>
                  <span class="value">{currentServer()?.region}</span>
                </div>
                <Show when={currentServer()?.latency}>
                  <div class="info-item">
                    <span class="label">Latency:</span>
                    <span class="value">{currentServer()?.latency}ms</span>
                  </div>
                </Show>
              </div>
            </Show>
          </div>
        </Show>
      </div>

      <div class="app-footer">
        <Show when={hasApiKey()}>
          <button
            class="btn-secondary btn-small"
            onClick={() => {
              setHasApiKey(false);
              setServers([]);
              setSelectedServerId(null);
            }}
          >
            Change API Key
          </button>
        </Show>
      </div>
    </div>
  );
}
