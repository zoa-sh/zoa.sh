function openModal(imgSrc, imgAlt) {
  var modal = document.getElementById("imageModal");
  var modalImg = document.getElementById("modalImage");
  var captionText = document.getElementById("caption");
  modal.style.display = "block";
  modalImg.src = imgSrc;
  captionText.innerHTML = imgAlt;

  modal.onclick = closeModal;
}

function closeModal() {
  var modal = document.getElementById("imageModal");
  modal.style.display = "none";
}

