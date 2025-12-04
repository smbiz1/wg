import { Show } from 'solid-js';
import './ConnectionToggle.css';

interface Server {
  id: string;
  name: string;
  region: string;
  latency?: number;
}

interface ConnectionToggleProps {
  isConnected: boolean;
  isConnecting: boolean;
  currentServer: Server | null;
  onConnect: () => void;
  onDisconnect: () => void;
}

export default function ConnectionToggle(props: ConnectionToggleProps) {
  const handleToggle = () => {
    if (props.isConnecting) {
      return;
    }

    if (props.isConnected) {
      props.onDisconnect();
    } else {
      props.onConnect();
    }
  };

  return (
    <div class="connection-toggle">
      <div class="status-display">
        <div
          class={`status-indicator ${props.isConnected ? 'connected' : 'disconnected'}`}
        />
        <div class="status-text">
          <Show
            when={props.isConnected}
            fallback={
              <span class="status-label">Disconnected</span>
            }
          >
            <span class="status-label">Connected</span>
            <Show when={props.currentServer}>
              <span class="status-server">{props.currentServer?.name}</span>
            </Show>
          </Show>
        </div>
      </div>

      <button
        onClick={handleToggle}
        disabled={props.isConnecting}
        class={`toggle-button ${props.isConnected ? 'connected' : 'disconnected'}`}
      >
        <Show when={props.isConnecting} fallback={
          <span>{props.isConnected ? 'Disconnect' : 'Connect'}</span>
        }>
          <span class="spinner" />
          <span>{props.isConnected ? 'Disconnecting...' : 'Connecting...'}</span>
        </Show>
      </button>
    </div>
  );
}
