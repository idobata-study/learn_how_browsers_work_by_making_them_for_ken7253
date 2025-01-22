import "@std/dotenv/load";

const PORT = parseInt(Deno.env.get("PORT") ?? Deno.args[0] ?? "8000", 10);
const serverController = new AbortController();
const decoder = new TextDecoder();

Deno.serve(
  { port: PORT, signal: serverController.signal },
  async (req: Request): Promise<Response> => {
    console.clear();
    console.log("==[Deno]===========================");
    console.log(req);
    console.log("===================================");

    const url = URL.parse(req.url, `http://localhost:${PORT}`);

    if (url === null) {
      return new Response("Invalid URL", { status: 400 });
    } else {
      if (url.pathname.endsWith("/")) {
        url.pathname = `${url.pathname}index.html`;
      }
      url.pathname = `${Deno.cwd()}/public${url.pathname}`;
      console.log(`[${req.method}]: ${url.pathname}`);
      try {
        const data = await Deno.readFile(url.pathname);
        const text = decoder.decode(data);
        return new Response(text, { status: 200 });
      } catch (error) {
        if (error instanceof Deno.errors.NotFound) {
          console.log("[FS]: Not Found");
          return new Response("Not Found", { status: 404 });
        } else if (Deno.errors.PermissionDenied) {
          console.log("[FS]: Permission Denied");
          return new Response("Permission Denied", { status: 403 });
        }
        console.error("[FS]: Internal Server Error");
        return new Response("Internal Server Error", { status: 500 });
      }
    }
  }
);
