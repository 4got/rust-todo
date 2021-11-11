'use strict';

const onEnter =
  (cb) =>
  (e = window.event) => {
    const keyCode = e.code || e.key;
    if (keyCode == 'Enter') {
      return cb();
    }
  };
const toUpdate = [];
const toUpdateLists = [];
const saveButton = document.querySelector('[onclick="save()"]');
const reloadButton = document.querySelector('[onclick="reload()"]');
const input = (id) => document.querySelector(`[name="${id}"]`);
const submitAction = (action) => {
  input('action').value = action;
  input('action').form.submit();
};
const check = (index) => {
  input('index').value = index;
  submitAction('check');
};
const uncheck = (index) => {
  input('index').value = index;
  submitAction('uncheck');
};
const remove = (index) => {
  input('index').value = index;
  submitAction('remove');
};
const closeSelect = (index) => {
  // const btn = window.event.target;
  const select = document.querySelector(`[data-select="${index}"]`);
  select.style.display = 'none';
};
const useSelect = (index) => {
  // const btn = window.event.target;
  const select = document.querySelector(`[data-select="${index}"]`);
  select.style.display = '';

  const closeWhenNotSelect = () => {
    if (select !== event.target && !select.contains(event.target)) {
      closeSelect(index);
      removeEventListener('click', closeWhenNotSelect);
    }
  };

  // setTimeout(() => addEventListener("click", closeWhenNotSelect), 0);
  addEventListener('click', closeWhenNotSelect);
  window.event.stopPropagation();
};
const markAs = (index, mark) => {
  input('mark_as').value = mark;
  input('index').value = index;
  submitAction('mark_as');
};
const rewrite = (index) => {
  saveButton.style.display = '';
  reloadButton.style.display = '';
  index = parseInt(index);
  const content = window.event.target.value;
  const found = toUpdate.find((t) => t.index === index);
  if (found) {
    found.content = content;
  } else toUpdate.push({ index, content });
};
const rename = (index) => {
  saveButton.style.display = '';
  reloadButton.style.display = '';
  index = parseInt(index);
  const name = window.event.target.value;
  const found = toUpdateLists.find((t) => t.index === index);
  if (found) {
    found.name = name;
  } else toUpdateLists.push({ index, name });
};
const move = (from, to) => {
  input('index').value = from;
  input('move_to').value = to;
  submitAction('move');
};
const moveToList = (from, to) => {
  input('index').value = from;
  input('move_to_list').value = to;
  submitAction('move_to_list');
};
const removeList = (id) => {
  input('list_id').value = id;
  submitAction('remove_list');
};
const reload = () => window.location.reload();
const save = () => {
  input('update').value = JSON.stringify(toUpdate);
  input('update_lists').value = JSON.stringify(toUpdateLists);
  submitAction('save');
};
const onEnterNew = (listId) => {
  const e = window.event;
  onEnter(() => {
    input('list_id').value = listId;
    input('new').value = e.target.value;
    submitAction('new');
  })();
};
const onEnterSave = (e = window.event) => {
  onEnter(save)();
};
const createList = () => {
  submitAction('new_list');
};

const slider = () => {
  const slides = [...document.querySelectorAll('[data-slide]')];
  const slideLinks = [...document.querySelectorAll('[data-slidelink]')];
  const showSlide = (i) => {
    if (!slides[i]) {
      if (i > 0) return showSlide(i - 1);
      else return;
    }
    slides.forEach((slide) => (slide.style.display = 'none'));
    slides[i].style.display = '';

    slideLinks.forEach((slide) => slide.classList.remove('active'));
    slideLinks[i].classList.add('active');
    localStorage.setItem('todolist:active_list_id', i);
  };

  slideLinks.forEach((link, i) => (link.onclick = () => showSlide(i)));
  showSlide(localStorage.getItem('todolist:active_list_id'));
};
window.addEventListener('DOMContentLoaded', slider);
