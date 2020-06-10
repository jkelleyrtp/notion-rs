#%%
from notion.client import NotionClient

# Obtain the `token_v2` value by inspecting your browser cookies on a logged-in session on Notion.so
client = NotionClient(token_v2="ebceea693d3c1ce55207c6abb62e39a6409a3c131834ed24ee3204bf9ddd2747b2067fd676f8a6f3f1e3611c0ba9db5a3f1528b38bac6ee0cbdc4fe81a9ebaa23a45079061b637be756f21dfe9af")
#%%
# Replace this URL with the URL of the page you want to edit
page = client.get_block("https://www.notion.so/jonathankelley/Interview-the-people-get-the-customers-meet-the-demand-46f9f915482a43adb0ca7ca4005255d9")


#%%
print("The old title is:", page.title)

#%%

# Note: You can use Markdown! We convert on-the-fly to Notion's internal formatted text data structure.
page.title = "The title has now changed, and has *live-updated* in the browser!"

