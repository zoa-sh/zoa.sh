document.addEventListener('DOMContentLoaded', function() {
  const showMoreButton = document.getElementById('showMoreProjects');
  const projectList = document.getElementById('projectList');

  if (showMoreButton) {
    showMoreButton.addEventListener('click', function() {
      const hiddenProjects = projectList.querySelectorAll('.project-item.hidden');
      
      hiddenProjects.forEach(project => {
        project.classList.remove('hidden');
      });

      showMoreButton.style.display = 'none';
    });
  }
});
