import { createSignal } from 'solid-js';
import './ApiKeyInput.css';

interface ApiKeyInputProps {
  onSubmit: (apiKey: string) => void;
}

export default function ApiKeyInput(props: ApiKeyInputProps) {
  const [apiKey, setApiKey] = createSignal('');
  const [isSubmitting, setIsSubmitting] = createSignal(false);

  const handleSubmit = async (e: Event) => {
    e.preventDefault();

    if (!apiKey().trim()) {
      return;
    }

    setIsSubmitting(true);
    await props.onSubmit(apiKey().trim());
    setIsSubmitting(false);
  };

  return (
    <div class="api-key-input-container">
      <div class="welcome-message">
        <h2>Welcome!</h2>
        <p>Enter your API key from the dashboard to get started.</p>
      </div>

      <form onSubmit={handleSubmit} class="api-key-form">
        <div class="form-group">
          <label for="api-key">API Key</label>
          <input
            type="text"
            id="api-key"
            value={apiKey()}
            onInput={(e) => setApiKey(e.currentTarget.value)}
            placeholder="Paste your API key here"
            disabled={isSubmitting()}
            autocomplete="off"
            class="api-key-input"
          />
        </div>

        <button
          type="submit"
          class="btn-primary"
          disabled={!apiKey().trim() || isSubmitting()}
        >
          {isSubmitting() ? (
            <>
              <span class="spinner" /> Validating...
            </>
          ) : (
            'Continue'
          )}
        </button>
      </form>

      <div class="help-text">
        <p>
          Don't have an API key?{' '}
          <a href="#" onClick={(e) => {
            e.preventDefault();
            // TODO: Open your actual dashboard URL
            window.open('https://dashboard.example.com/vpn', '_blank');
          }}>
            Get one from the dashboard
          </a>
        </p>
      </div>
    </div>
  );
}
