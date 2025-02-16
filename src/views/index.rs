use warp::Rejection;
use warp::Reply;

pub async fn index() -> Result<impl Reply, Rejection> {
    let html = r##"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Linkie - URL Shortener</title>
            <meta name="description" content="Linkie is a powerful URL shortener that transforms your long URLs into short, shareable links.">
            <meta name="keywords" content="URL shortener, Linkie, shorten URL, shareable links, QR code generator">
            <meta name="author" content="codewithwan">
            <link rel="canonical" href="https://linkie.my.id">
            <meta property="og:title" content="Linkie - URL Shortener">
            <meta property="og:description" content="Transform your long URLs into short, shareable links with Linkie.">
            <meta property="og:url" content="https://linkie.my.id">
            <meta property="og:type" content="website">
            <meta property="og:image" content="https://linkie.my.id/og-image.png">
            <meta name="twitter:card" content="summary_large_image">
            <meta name="twitter:title" content="Linkie - URL Shortener">
            <meta name="twitter:description" content="Transform your long URLs into short, shareable links with Linkie.">
            <meta name="twitter:image" content="https://linkie.my.id/twitter-image.png">
            <link href="https://cdnjs.cloudflare.com/ajax/libs/tailwindcss/2.2.19/tailwind.min.css" rel="stylesheet">
            <style>
                @keyframes pulse {
                    0%, 100% { transform: scale(1); }
                    50% { transform: scale(1.05); }
                }
                .loading {
                    animation: spin 1s linear infinite;
                }
                @keyframes spin {
                    from { transform: rotate(0deg); }
                    to { transform: rotate(360deg); }
                }
                .fade-in {
                    animation: fadeIn 0.5s ease-in;
                }
                @keyframes fadeIn {
                    from { opacity: 0; transform: translateY(10px); }
                    to { opacity: 1; transform: translateY(0); }
                }
            </style>
        </head>
        <body class="bg-gray-50 min-h-screen">
            <div class="max-w-3xl mx-auto px-4 py-16">
                <div class="bg-white rounded-2xl shadow-xl p-8 transition-all duration-300 hover:shadow-2xl">
                    <div class="text-center mb-8">
                        <h1 class="text-4xl font-bold text-gray-800 mb-2">URL Shortener</h1>
                        <p class="text-gray-600">Transform your long URLs into short, shareable links</p>
                    </div>

                    <div class="space-y-4">
                        <div class="relative">
                            <input 
                                type="text" 
                                id="url" 
                                class="w-full px-4 py-3 rounded-lg border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-all duration-200 text-gray-800 placeholder-gray-400"
                                placeholder="Paste your long URL here..."
                            >
                            <div id="validation-message" class="absolute left-0 top-full mt-1 text-sm text-red-500 hidden">
                                Please enter a valid URL
                            </div>
                        </div>

                        <button 
                            onclick="shortenUrl()"
                            id="shorten-button"
                            class="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-3 px-4 rounded-lg transition-all duration-200 transform hover:scale-[1.02] focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                        >
                            <span id="button-text">Shorten URL</span>
                            <svg id="loading-spinner" class="hidden loading w-5 h-5 text-white inline" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                            </svg>
                        </button>
                    </div>

                    <div id="result" class="hidden mt-8 p-6 bg-gray-50 rounded-lg border border-gray-200 fade-in">
                        <div class="flex flex-col space-y-2">
                            <label class="text-sm font-medium text-gray-700">Your shortened URL:</label>
                            <div class="flex items-center space-x-2">
                                <input 
                                    type="text" 
                                    id="shortened-url" 
                                    class="flex-1 px-3 py-2 rounded-lg border border-gray-300 bg-white text-gray-800 read-only:bg-gray-50"
                                    readonly
                                >
                                <button 
                                    onclick="copyToClipboard()"
                                    class="px-4 py-2 bg-gray-800 text-white rounded-lg hover:bg-gray-700 transition-colors duration-200"
                                >
                                    Copy
                                </button>
                            </div>
                        </div>
                        <div class="mt-4">
                            <label class="text-sm font-medium text-gray-700">QR Code:</label>
                            <div class="flex flex-col items-center space-y-2">
                                <img id="qr-code" src="#" alt="QR Code" class="w-32 h-32">
                                <a 
                                    id="download-link" 
                                    href="#" 
                                    download="qrcode_url.png" 
                                    class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors duration-200"
                                >
                                    Download QR Code
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <footer class="text-center mt-8 text-gray-600">
                <p>&copy; <span id="current-year"></span> <a href="https://github.com/codewithwan/rust-url-shortener" class="text-blue-600 hover:underline">codewithwan</a>. All rights reserved.</p>
            </footer>
            <script>
                document.getElementById('current-year').textContent = new Date().getFullYear();

                async function shortenUrl() {
                    const urlInput = document.getElementById('url');
                    const button = document.getElementById('shorten-button');
                    const buttonText = document.getElementById('button-text');
                    const spinner = document.getElementById('loading-spinner');
                    const validationMessage = document.getElementById('validation-message');
                    const result = document.getElementById('result');

                    try {
                        new URL(urlInput.value);
                        validationMessage.classList.add('hidden');
                    } catch {
                        validationMessage.classList.remove('hidden');
                        urlInput.focus();
                        return;
                    }

                    button.disabled = true;
                    buttonText.classList.add('hidden');
                    spinner.classList.remove('hidden');

                    try {
                        const response = await fetch('/shorten', {
                            method: 'POST',
                            headers: {
                                'Content-Type': 'application/json',
                            },
                            body: JSON.stringify({ url: urlInput.value }),
                        });
                        const data = await response.json();
                        
                        document.getElementById('shortened-url').value = data.short_url;
                        document.getElementById('qr-code').src = data.qr_code;
                        document.getElementById('download-link').href = data.qr_code;
                        result.classList.remove('hidden');
                    } catch (error) {
                        alert('An error occurred. Please try again.');
                    } finally {
                        button.disabled = false;
                        buttonText.classList.remove('hidden');
                        spinner.classList.add('hidden');
                    }
                }

                function copyToClipboard() {
                    const shortenedUrl = document.getElementById('shortened-url');
                    shortenedUrl.select();
                    document.execCommand('copy');
                    
                    const copyButton = document.querySelector('button:last-of-type');
                    const originalText = copyButton.textContent;
                    copyButton.textContent = 'Copied!';
                    setTimeout(() => {
                        copyButton.textContent = originalText;
                    }, 2000);
                }

                document.getElementById('url').addEventListener('keypress', function(e) {
                    if (e.key === 'Enter') {
                        shortenUrl();
                    }
                });
            </script>
        </body>
        </html>
    "##;
    Ok(warp::reply::html(html))
}