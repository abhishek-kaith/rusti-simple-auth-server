const observer = new MutationObserver((mutationsList) => {
  for (let mutation of mutationsList) {
    console.log(mutation.type === "attributes");
    if (mutation.type === "attributes") {
      const element = mutation.target as HTMLElement;
      console.log(element);
    }
  }
});

observer.observe(document.body, {
  attributes: true,
  subtree: true,
  characterData: true,
});
