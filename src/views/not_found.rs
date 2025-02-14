use warp::Rejection;
use warp::Reply;

pub async fn not_found() -> Result<impl Reply, Rejection> {
    let html = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>404 Not Found</title>
            <link href="https://cdnjs.cloudflare.com/ajax/libs/tailwindcss/2.2.19/tailwind.min.css" rel="stylesheet">
        </head>
        <body class="bg-gray-50 min-h-screen flex items-center justify-center">
            <div class="text-center">
                <h1 class="text-6xl font-bold text-gray-800 mb-4">404</h1>
                <p class="text-gray-600 mb-8">The page you are looking for was not found.</p>
                <a href="/" class="bg-blue-600 hover:bg-blue-700 text-white font-medium py-3 px-6 rounded-lg transition-all duration-200">Go to Home</a>
            </div>
        </body>
        </html>
    "#;
    Ok(warp::reply::html(html))
}
