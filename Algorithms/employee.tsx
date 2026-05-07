import React from "react";

const EMAIL_REGEX = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
const ONLY_LETTERS_REGEX = /^\p{L}+$/u;
const EXACTLY_SIX_DIGITS_REGEX = /^\d{6}$/;
const formObject = {
  name: { value: "", error: "", touched: false },
  email: { value: "", error: "", touched: false },
  employeeId: { value: "", error: "", touched: false },
  joiningDate: { value: "", error: "", touched: false }
}
function EmployeeValidationForm() {

  const [formData, setFormData] = useState();

  const setErrorState = (field, message) => {
    setErrors((prev) => {
      [field]: {
        ...prev[field],
        error: message
      }
    })
  }

  const validate = (name, value) => {
    switch(name):
      case "name":
        if(value)
      case "email":
        if(!EMAIL_REGEX.test(value)) {
          setErrorState(name, )
        }
      case "employeeId":
        if(!EXACTLY_SIX_DIGITS_REGEX.test(value)){
          setErrors((prev) => {
            ...prev,
            employeeId: "Must be exactly 6 numeric digits."
          })
        }
      case "joiningDate":
        if(!isNaN(value)){
          const today = new Date();
          const date = new Date(value);
          if (date > today) {
            setErrors((prev) => {
              ...prev,
              name: "Cannot be set to a date in the future."
          }
      })
    }
  }

  const onChange = (e) => {
    const { key, value } = e.target;
    setFormFields((prev) => ({
      ...prev,
      [key]: value,
    }))
  }
  return (
    <div className="layout-column align-items-center mt-20 ">
      <div className="layout-column align-items-start mb-10 w-50" data-testid="input-name">
        <input
          className="w-100"
          type="text"
          name="name"
          value={formFields.name}
          onChange={onChange}
          placeholder="Name"
          data-testid="input-name-test"
        />
        {errors.name && <p className="error mt-2">
          Name must be at least 4 characters long and only contain letters and spaces
        </p> }
      </div>
      <div className="layout-column align-items-start mb-10 w-50" data-testid="input-email">
        <input
          className="w-100"
          type="text"
          name="email"
          value={formFields.email}
          onChange={onchange}
          placeholder="Email"
        />
        { errors.email && <p className="error mt-2">Email must be a valid email address</p> }
      </div>
      <div className="layout-column align-items-start mb-10 w-50" data-testid="input-employee-id">
        <input
          className="w-100"
          type="text"
          name="employeeId"
          value={formFields.employeeId}
          onchange={onchange}
          placeholder="Employee ID"
        />
        {errors.employeeId && <p className="error mt-2">Employee ID must be exactly 6 digits</p> }
      </div>
      <div className="layout-column align-items-start mb-10 w-50" data-testid="input-joining-date">
        <input
          className="w-100"
          type="date"
          name="joiningDate"
          value="2023-12-04"
          placeholder="Joining Date"
        />
        { errors.joiningDate && <p className="error mt-2">Joining Date cannot be in the future</p> }
      </div>
      <button data-testid="submit-btn" type="submit" onClick={onSubmit}>
        Submit
      </button>
    </div>
  );
}

export default EmployeeValidationForm;

